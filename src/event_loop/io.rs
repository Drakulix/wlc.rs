use super::Event;

use ffi;
use libc;

use std::mem;
use std::os::unix::io::AsRawFd;
use std::rc::{Rc, Weak};

/// Type representing a registered event source
///
/// Dropping this will deregister the event source and drop the associated io
/// struct and callback
pub struct EventSource(*mut ffi::wlc_event_source, Rc<()>);

/// Implement this to react to events
///
/// An Implementation for `FnMut` is provided, so you may use a (anonymous)
/// function instead
pub trait IoCallback<R> {
    /// called when a new event happened
    fn ready(&mut self, io: &mut R, event: Event::Flags);
}
impl<R, F> IoCallback<R> for F
    where F: FnMut(&mut R, Event::Flags)
{
    fn ready(&mut self, io: &mut R, event: Event::Flags) {
        self(io, event)
    }
}

impl Drop for EventSource {
    fn drop(&mut self) {
        unsafe {
            ffi::wlc_event_source_remove(self.0);
        }
    }
}

/// Register an IO type implementing `AsRawFd` for event loop callbacks on
/// specified events
///
/// # Safety
/// Dont call this function on another thread, then the main thread.
pub fn event_loop_add_io<R: AsRawFd, T: IoCallback<R>>(io: R, mask: Event::Flags, callback: T)
                                                       -> EventSource {
    let notification = Rc::new(());

    let event_source = unsafe {
        ffi::wlc_event_loop_add_fd(io.as_raw_fd(),
                                   mask.bits(),
                                   Some(event_loop_io_cb::<R, T>),
                                   Box::into_raw(Box::new((io, callback, Rc::downgrade(&notification)))) as
                                   *mut _)
    };

    EventSource(event_source, notification)
}

#[cfg_attr(feature = "cargo-clippy", allow(deref_addrof))]
unsafe extern "C" fn event_loop_io_cb<R, T: IoCallback<R>>(_fd: i32, mask: u32, userdata: *mut libc::c_void)
                                                           -> i32 {
    let mut boxed: Box<(R, T, Weak<()>)> = Box::from_raw(userdata as *mut _);

    let _guard = match boxed.2.upgrade() {
        Some(val) => val,
        None => return 0, //drops io and callback
    };

    {
        let (ref mut io, ref mut callback, _) = *&mut *boxed;
        callback.ready(io, Event::Flags::from_bits_truncate(mask))
    };

    mem::forget(boxed);

    0
}
