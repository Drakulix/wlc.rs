use ffi;
use libc;

use std::mem;
use std::rc::{Rc, Weak};
use std::time::Duration;

/// Type representing a registered Timer
///
/// Dropping this will deregister the timer and drop the associated callback
pub struct Timer(*mut ffi::wlc_event_source, Rc<()>);

/// Implement this to react to timer events
///
/// An Implementation for `FnMut` is provided, so you may use a (anonymous)
/// function instead
pub trait TimerCallback: Sized {
    /// called once for every call to update after the specified delay
    fn fire(&mut self);
}
impl<F> TimerCallback for F
    where F: FnMut()
{
    fn fire(&mut self) {
        self()
    }
}

impl Timer {
    /// Call to schedule the next fire event after a given Duration
    pub fn update(&mut self, at: &Duration) {
        unsafe {
            ffi::wlc_event_source_timer_update(self.0,
                                               ((at.as_secs() * 1000u64) as u32 +
                                                at.subsec_nanos() / 1_000_000u32) as
                                               i32)
        };
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        unsafe {
            ffi::wlc_event_source_remove(self.0);
        }
    }
}

/// Created a registered Timer with a given Callback
///
/// # Safety
/// Don't call this function on another thread, then the main thread.
pub fn event_loop_add_timer<T: TimerCallback>(callback: T) -> Timer {
    let notification = Rc::new(());

    let event_source =
        unsafe {
            ffi::wlc_event_loop_add_timer(Some(event_loop_timer_cb::<T>),
                                          Box::into_raw(Box::new((callback, Rc::downgrade(&notification)))) as
                                          *mut _)
        };

    Timer(event_source, notification)
}

unsafe extern "C" fn event_loop_timer_cb<T: TimerCallback>(userdata: *mut libc::c_void) -> i32 {
    let mut boxed: Box<(T, Weak<()>)> = Box::from_raw(userdata as *mut _);

    let _guard = match boxed.1.upgrade() {
        Some(val) => val,
        None => return 0,  //drops callback
    };

    boxed.0.fire();

    mem::forget(boxed);

    0
}
