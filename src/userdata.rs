use {Output, View};

use ffi;
use libc::c_void;
use output::handle as output_handle;

use std::any::Any;
use std::mem;
use std::ptr;
use std::rc::Rc;
use view::handle as view_handle;

/// Assiciated user-provided data
///
/// Types implemeting Handle (Output, View) may hold a pointer to user-data.
/// This Trait handles storing and receiving arbitrary data for such Types.
pub trait Handle {
    /// Sets any data as user pointer consuming the data.
    ///
    /// Clears old user data before inserting new one (`drop`s it correctly).
    fn set_user_data<T: Any>(&self, data: T);

    /// Receive a shared reference to the user data of a given type, if user
    /// data exists.
    ///
    /// Returns `None` if no user data is set.
    ///
    /// # Safety
    /// While the function confirms that userdata does actually exists,
    /// it does not verify that `T` is the correct Type for the data making it
    /// unsafe if the wrong type is used.
    unsafe fn user_data<T: Any>(&self) -> Option<Rc<T>>;

    /// Tries to take the userdata exclusively and removes it from the Handle.
    ///
    /// Returns `None` if no data exists or shared references do exist that
    /// make taking impossible.
    ///
    /// # Safety
    /// While the function confirms that userdata does actually exists,
    /// it does not verify that `T` is the correct Type for the data making it
    /// unsafe if the wrong type is used.
    unsafe fn try_take_user_data<T: Any>(&self) -> Option<T>;

    /// Clears currently set user data (`Drop` gets called, after all
    /// references are dropped).
    ///
    /// This is safe and a no-op if no user data exists
    fn clear_user_data(&self);
}

impl Handle for Output {
    fn set_user_data<T: Any>(&self, data: T) {
        self.clear_user_data(); // drop

        let boxed = Box::new(Rc::new(data));
        unsafe {
            ffi::wlc_handle_set_user_data(output_handle(self), Box::into_raw(boxed) as *const c_void);
        }
    }

    unsafe fn user_data<T: Any>(&self) -> Option<Rc<T>> {
        let ptr = ffi::wlc_handle_get_user_data(output_handle(self));
        let boxed: Box<Rc<T>> = if ptr.is_null() {
            return None;
        } else {
            Box::from_raw(ptr as *mut Rc<T>)
        };

        // inc ref count on return
        let result = Some((*boxed).clone());
        mem::forget(boxed);
        result
    }

    unsafe fn try_take_user_data<T: Any>(&self) -> Option<T> {
        match self.user_data::<T>() {
            Some(rc) => {
                self.clear_user_data();
                match Rc::try_unwrap(rc) {
                    Ok(result) => Some(result),
                    Err(rc) => {
                        ffi::wlc_handle_set_user_data(output_handle(self),
                                                      Box::into_raw(Box::new(rc)) as *const c_void);
                        None
                    }
                }
            }
            None => None,
        }
    }

    fn clear_user_data(&self) {
        let ptr = unsafe { ffi::wlc_handle_get_user_data(output_handle(self)) };
        let _drop: Box<Any> = if ptr.is_null() {
            return;
        } else {
            unsafe { Box::from_raw(ptr as *mut Any) }
        };
        unsafe {
            ffi::wlc_handle_set_user_data(output_handle(self), ptr::null_mut());
        }
    }
}


impl Handle for View {
    fn set_user_data<T: Any>(&self, data: T) {
        self.clear_user_data(); // drop

        let boxed = Box::new(Rc::new(data));
        unsafe {
            ffi::wlc_handle_set_user_data(view_handle(self), Box::into_raw(boxed) as *const c_void);
        }
    }

    unsafe fn user_data<T: Any>(&self) -> Option<Rc<T>> {
        let ptr = ffi::wlc_handle_get_user_data(view_handle(self));
        let boxed: Box<Rc<T>> = if ptr.is_null() {
            return None;
        } else {
            Box::from_raw(ptr as *mut Rc<T>)
        };

        // inc ref count on return
        let result = Some((*boxed).clone());
        mem::forget(boxed);
        result
    }

    unsafe fn try_take_user_data<T: Any>(&self) -> Option<T> {
        match self.user_data::<T>() {
            Some(rc) => {
                self.clear_user_data();
                match Rc::try_unwrap(rc) {
                    Ok(result) => Some(result),
                    Err(rc) => {
                        ffi::wlc_handle_set_user_data(view_handle(self),
                                                      Box::into_raw(Box::new(rc)) as *const c_void);
                        None
                    }
                }
            }
            None => None,
        }
    }

    fn clear_user_data(&self) {
        let ptr = unsafe { ffi::wlc_handle_get_user_data(view_handle(self)) };
        let _drop: Box<Any> = if ptr.is_null() {
            return;
        } else {
            unsafe { Box::from_raw(ptr as *mut Any) }
        };
        unsafe {
            ffi::wlc_handle_set_user_data(view_handle(self), ptr::null_mut());
        }
    }
}
