//! Experimental Input API
//!
//! ## wlc's original description:
//!
//! Very recent stuff, things may change.
//!
//! XXX: This api is dumb and assumes there is only single xkb state and keymap.
//! In case of multiple keyboards, we want to each keyboard have own state
//! and layout.
//!      Thus we need `wlc_handle` for keyboards eventually.

mod consts;

/// Keyboard related functionality
pub mod keyboard {
    pub use super::consts::Key;
    use Modifiers;
    use ffi;
    use std::mem;

    use std::slice;
    pub use xkbcommon::xkb::{Keymap, Keysym, State};

    pub use xkbcommon::xkb::keysyms as Keysyms;

    /// Internal `xkb_state` exposed. You can use it to do more advanced key
    /// handling.
    /// However you should avoid messing with its state
    pub fn xkb_state() -> State {
        unsafe { State::from_raw_ptr(ffi::wlc_keyboard_get_xkb_state() as *mut _) }
    }

    /// Internal `xkb_keymap` exposed. You can use it to do more advanced key
    /// handling
    pub fn xkb_keymap() -> Keymap {
        unsafe { Keymap::from_raw_ptr(ffi::wlc_keyboard_get_xkb_keymap() as *mut _) }
    }

    /// Get currently held keys
    pub fn current_keys<'a>() -> &'a [Key] {
        let mut len = 0;
        unsafe {
            let ptr = ffi::wlc_keyboard_get_current_keys(&mut len as *mut _) as *const Key;
            slice::from_raw_parts(ptr, len)
        }
    }

    /// Utility function to convert raw keycode to keysym. Passed modifiers may
    /// transform the key
    pub fn keysym_for_key(key: Key, modifiers: Modifiers) -> Keysym {
        unsafe {
            ffi::wlc_keyboard_get_keysym_for_key(key as u32,
                                                 mem::transmute::<&Modifiers,
                                                                  &ffi::wlc_modifiers>(&modifiers) as
                                                 *const _)
        }
    }

    /// Utility function to convert raw keycode to Unicode/UTF-32 codepoint.
    /// Passed modifiers may transform the key
    pub fn utf32_for_key(key: Key, modifiers: Modifiers) -> u32 {
        unsafe {
            ffi::wlc_keyboard_get_utf32_for_key(key as u32,
                                                mem::transmute::<&Modifiers,
                                                                 &ffi::wlc_modifiers>(&modifiers) as
                                                *const _)
        }
    }
}

/// Pointer related functionality
pub mod pointer {
    pub use super::consts::Button;
    use Point;
    use ffi;
    use std::mem;

    /// Get current pointer position
    pub fn position() -> Point {
        unsafe {
            let mut point: ffi::wlc_point = mem::uninitialized();
            ffi::wlc_pointer_get_position(&mut point as *mut _);
            Point::from_ffi(&point)
        }
    }

    /// Set current pointer position
    pub fn set_position(position: Point) {
        unsafe { ffi::wlc_pointer_set_position(&position.into_ffi() as *const _) }
    }
}
