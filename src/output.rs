use {Size, View, Visibility};

use ffi;
use libc;

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ptr;
use std::rc::{Rc, Weak};
use std::slice;

pub struct NotRequiredThreadSafe<T>(pub T);
unsafe impl<T> Send for NotRequiredThreadSafe<T> {}
unsafe impl<T> Sync for NotRequiredThreadSafe<T> {}

lazy_static! {
    pub static ref OUTPUTS:
        NotRequiredThreadSafe<RefCell<HashMap<libc::uintptr_t, Rc<()>>>>
        = NotRequiredThreadSafe(RefCell::new(HashMap::new()));
}

/// An Output managed by Wlc
#[repr(C)]
pub struct Output;

#[cfg(not(feature = "unsafe-stable"))]
impl !Sync for Output {}
#[cfg(not(feature = "unsafe-stable"))]
impl !Send for Output {}

impl fmt::Debug for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Output {{ id: {} }}", handle(self))
    }
}

impl Output {
    /// Get all outputs currently active outputs
    ///
    /// # Safety
    /// This function is unsafe, because it creates an [unbound
    /// lifetime](https://doc.rust-lang.org/stable/nomicon/unbounded-lifetimes.
    /// html).
    /// No output lives forever and might be disconnected at any time
    /// A disconnection is signaled by `Callback::output_destroyed` and the
    /// Output
    /// is deallocated shortly after.
    /// Because of this using this function one may create an invalid Output
    /// reference.
    ///
    /// Dont call this function on another thread, then the main thread
    ///
    /// See `Output::with_all_outputs` for a safe variant
    pub unsafe fn all_outputs<'a>() -> &'a [Output] {
        let mut size = 0usize;
        let output_ptr = ffi::wlc_get_outputs(&mut size) as *mut _;
        slice::from_raw_parts_mut(output_ptr, size)
    }

    /// Safe version of `Output::all_outputs`
    /// Work with all active outputs in a short-lived scope
    ///
    /// # Safety
    /// By enforcing a rather harsh limit on the lifetime of the outputs
    /// to a short lived scope of an anonymous function,
    /// this function makes sure no output lives longer then it exists.
    ///
    /// Dont call this function on another thread, then the main thread
    pub fn with_all_outputs<F, R>(runner: F) -> R
        where F: FnOnce(&[Output]) -> R
    {
        let outputs = unsafe { Output::all_outputs() };
        runner(outputs)
    }

    /// Get currently focused output
    ///
    /// # Safety
    /// This function is unsafe, because it creates an [unbound
    /// lifetime](https://doc.rust-lang.org/stable/nomicon/unbounded-lifetimes.
    /// html).
    /// No output lives forever and might be disconnected at any time
    /// A disconnection is signaled by `Callback::output_destroyed` and the
    /// Output is
    /// deallocated shortly after.
    /// Because of this using this function one may create an invalid Output
    /// reference.
    ///
    /// Dont call this function on another thread, then the main thread
    ///
    /// See `Output::with_focused_output` for a safe variant
    pub unsafe fn focused_output<'a>() -> &'a Output {
        from_handle(ffi::wlc_get_focused_output())
    }

    /// Safe version of `Output::focused_output`
    /// Work with currently focused outputs in a short-lived scope
    ///
    /// # Safety
    /// By enforcing a rather harsh limit on the lifetime of the output
    /// to a short lived scope of an anonymous function,
    /// this function makes sure the output does not live longer then it exists.
    ///
    /// Dont call this function on another thread, then the main thread
    pub fn with_focused_output<F, R>(runner: F) -> R
        where F: FnOnce(&Output) -> R
    {
        let output = unsafe { Output::focused_output() };
        runner(output)
    }

    /// Set no output focused
    ///
    /// # Safety
    /// Dont call this function on another thread, then the main thread
    pub fn set_no_focus() {
        unsafe { ffi::wlc_output_focus(mem::transmute::<*const libc::c_void, libc::uintptr_t>(ptr::null())) }
    }

    /// Get output name
    pub fn name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(ffi::wlc_output_get_name(handle(self))).to_string_lossy() }
    }

    /// Get sleep state
    pub fn is_sleeping(&self) -> bool {
        unsafe { ffi::wlc_output_get_sleep(handle(self)) }
    }

    /// Wake up / send output to sleep
    pub fn set_sleeping(&self, sleep: bool) {
        unsafe { ffi::wlc_output_set_sleep(handle(self), sleep) }
    }

    /// Get real resolution.
    ///
    /// Resolution applied by either `output.set_resolution` call or initially.
    /// Do not use this for coordinate boundary.
    pub fn resolution(&self) -> Size {
        Size::from_ffi(unsafe { &*ffi::wlc_output_get_resolution(handle(self)) })
    }

    /// Get virtual resolution.
    ///
    /// Resolution with transformations applied for proper rendering for
    /// example on high density displays.
    /// Use this to figure out coordinate boundary.
    pub fn virtual_resolution(&self) -> Size {
        Size::from_ffi(unsafe { &*ffi::wlc_output_get_virtual_resolution(handle(self)) })
    }

    /// Set resolution
    pub fn set_resolution(&self, resolution: Size, scale: u32) {
        unsafe { ffi::wlc_output_set_resolution(handle(self), &resolution.into_ffi() as *const _, scale) }
    }

    /// Get scale factor
    pub fn scale(&self) -> u32 {
        unsafe { ffi::wlc_output_get_scale(handle(self)) }
    }

    /// Get current visibility bitmask
    pub fn visibility(&self) -> Visibility::Flags {
        Visibility::Flags::from_bits_truncate(unsafe { ffi::wlc_output_get_mask(handle(self)) })
    }

    /// Set visibility bitmask
    pub fn set_visibility(&self, slots: Visibility::Flags) {
        unsafe { ffi::wlc_output_set_mask(handle(self), slots.bits()) }
    }

    /// Get views in stack order
    pub fn views(&self) -> Vec<&View> {
        let mut size = 0usize;
        let view_ptr = unsafe { ffi::wlc_output_get_views(handle(self), &mut size) as *const _ };
        let slice = unsafe { slice::from_raw_parts(view_ptr, size) };
        slice.to_vec()
    }

    /// Sets the view stack
    pub fn set_views<'a, I: IntoIterator<Item = &'a View>>(&self, views: I) -> Result<(), Vec<&'a View>> {
        let mut vec: Vec<ffi::wlc_handle> = views.into_iter()
            .map(|x| unsafe { mem::transmute::<&View, ffi::wlc_handle>(x) })
            .collect();

        let len = vec.len();
        if unsafe { ffi::wlc_output_set_views(handle(self), vec.as_mut_ptr(), len) } {
            Ok(mem::forget(vec))
        } else {
            Err(vec.into_iter()
                .map(|x| unsafe { mem::transmute::<ffi::wlc_handle, &View>(x) })
                .collect())
        }
    }

    /// Focus output
    pub fn focus(&self) {
        unsafe { ffi::wlc_output_focus(handle(self)) }
    }

    /// Get the supported gamma ramp size of the `Output`
    pub fn gamma_size(&self) -> u16 {
        unsafe { ffi::wlc_output_get_gamma_size(handle(self)) }
    }

    /// Set gamma ramps for this `Output`
    ///
    /// `r`,`g` and `b` sizes should correspond to `gamma_size` returned value
    ///
    /// # Panic
    /// Panics if `r`, `g`, `b` have different sizes
    pub fn set_gamma(&self, r: &mut [u16], g: &mut [u16], b: &mut [u16]) {
        if r.len() != g.len() || r.len() != b.len() {
            panic!("Color ramps do not have the same size");
        }

        unsafe {
            ffi::wlc_output_set_gamma(handle(self),
                                      r.len() as u16,
                                      r.as_mut_ptr(),
                                      g.as_mut_ptr(),
                                      b.as_mut_ptr())
        }
    }

    /// Schedules output for rendering next frame.
    ///
    /// If output was already scheduled this is no-op,
    /// if output is currently rendering, it will render immediately after.
    #[cfg(feature = "render")]
    pub fn schedule_render(&self) {
        unsafe { ffi::wlc_output_schedule_render(handle(self)) }
    }

    /// Get a weak reference of the Output that may outlive its referenced
    /// output
    ///
    /// Since Output is always limited in its use by its lifetime, it is not
    /// very suitable for storing.
    /// This function allows you to optain a weak reference, that may outlive
    /// the output it is referencing.
    pub fn weak_reference(&self) -> WeakOutput {
        let mut outputs = OUTPUTS.0.borrow_mut();
        let ref_counter = outputs.entry(handle(self)).or_insert_with(|| Rc::new(()));
        WeakOutput(Rc::downgrade(ref_counter), handle(self))
    }
}


/// Weak reference to an output
///
/// Can be optained by `output.weak_reference()`
#[derive(Clone)]
pub struct WeakOutput(Weak<()>, ffi::wlc_handle);
impl WeakOutput {
    /// Upgrade your weak reference to an actual `Output`
    ///
    /// # Safety
    /// This function is unsafe, because it creates a lifetime bound to
    /// WeakOutput, which may live forever..
    /// But no output lives forever and might be disconnected at any time
    /// A disconnection is signaled by `Callback::output_destroyed` and the
    /// Output
    /// is deallocated shortly after.
    /// Because of this using this function one may create an invalid Output
    /// reference.
    ///
    /// See `WeakOutput::run` for a safe variant
    pub unsafe fn upgrade(&self) -> Option<&Output> {
        let test = self.0.clone().upgrade();
        match test {
            Some(_) => Some(from_handle(self.1)),
            None => None,
        }
    }

    /// Run a function on the referenced Output, if it still exists
    ///
    /// Returns the result of the function, if successful
    ///
    /// # Safety
    /// By enforcing a rather harsh limit on the lifetime of the output
    /// to a short lived scope of an anonymous function,
    /// this function makes sure the output does not live longer then it exists.
    pub fn run<F, R>(&self, runner: F) -> Option<R>
        where F: FnOnce(&Output) -> R
    {
        let output = unsafe { self.upgrade() };
        match output {
            Some(output) => Some(runner(output)),
            None => None,
        }
    }
}

impl fmt::Debug for WeakOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WeakOutput {{ id: {} }}", self.1)
    }
}

#[cfg(not(feature = "unsafe-stable"))]
impl !Send for WeakOutput {}
#[cfg(not(feature = "unsafe-stable"))]
impl !Sync for WeakOutput {}

pub fn from_handle<'a>(handle: ffi::wlc_handle) -> &'a mut Output {
    unsafe { &mut *(handle as *mut Output) }
}

pub fn handle(output: &Output) -> ffi::wlc_handle {
    unsafe { mem::transmute(output) }
}

impl PartialEq for Output {
    fn eq(&self, other: &Output) -> bool {
        handle(self) == handle(other)
    }
}
impl Eq for Output {}

impl PartialEq<WeakOutput> for Output {
    fn eq(&self, other: &WeakOutput) -> bool {
        handle(self) == other.1
    }
}

impl PartialEq<Output> for WeakOutput {
    fn eq(&self, other: &Output) -> bool {
        self.1 == handle(other)
    }
}

impl PartialEq for WeakOutput {
    fn eq(&self, other: &WeakOutput) -> bool {
        self.1 == other.1
    }
}
impl Eq for WeakOutput {}

use std::hash::*;

impl Hash for Output {
    fn hash<H: Hasher>(&self, state: &mut H) {
        handle(self).hash(state);
    }
}

impl Hash for WeakOutput {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}
