#![cfg_attr(not(feature = "unsafe-stable"), feature(optin_builtin_traits))]
#![allow(unknown_lints)]
#![deny(missing_docs, clippy)]

//! This crate provides idomatic and safe bindings to
//! [wlc](https://github.com/Cloudef/wlc)
//!
//!
//! wlc provides basic functionality to write a wayland compositor.
//!
//! Start your compositor by calling `wlc::init` with any struct implementing
//! `wlc::Callback`.
//! Handle `wlc::View` and `wlc::Output` references in your `Callback` struct
//! to customize your compositor.
//!
//! # Example
//!
//! ```rust,no_run
//! use wlc::*;
//!
//! struct Compositor;
//! impl Callback for Compositor
//! {
//!     fn view_created(&mut self, view: &View) -> bool
//!     {
//!         view.set_visibility(view.output().visibility());
//!         view.bring_to_front();
//!         view.focus();
//!         true
//!     }
//!
//!     fn view_focus(&mut self, view: &View, focus: bool)
//!     {
//!         view.set_state(ViewState::Activated, focus);
//!     }
//! }
//!
//! fn main()
//! {
//!     let _wlc = wlc::init(Compositor).unwrap();
//!     //_wlc goes out of scope and stars running until it terminates
//! }
//! ```

extern crate wlc_sys as ffi;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
extern crate num_traits;
#[macro_use]
extern crate log;
extern crate libc;
extern crate uinput_sys;
extern crate xkbcommon;
#[cfg(feature = "wayland")]
extern crate wayland_server;
#[cfg(feature = "serialization")]
extern crate serde;
#[cfg(feature = "serialization")]
#[cfg_attr(feature = "serialization", macro_use)]
extern crate serde_derive;

use num_traits::FromPrimitive;

use std::cell::UnsafeCell;
use std::error::Error as ErrorTrait;
use std::ffi::CStr;
use std::fmt;
use std::mem;

#[macro_use]
mod enum_primitive;

#[cfg(feature = "serialization")]
#[cfg_attr(feature = "serialization", macro_use)]
mod serialization;
#[cfg(not(feature = "serialization"))]
macro_rules! bitflags_serde {
    ( $name:ident { $($variant:ident, )* }) => {}
}
#[cfg(not(feature = "serialization"))]
macro_rules! enum_serde {
    ( $name:ident { $($variant:ident, )* }) => {}
}

mod output;
mod view;
mod types;
mod userdata;

pub mod event_loop;
pub mod input;
#[cfg(feature = "wayland")]
pub mod wayland;
#[cfg(feature = "render")]
pub mod render;

pub use self::output::{Output, WeakOutput};
use self::output::OUTPUTS;
#[cfg(feature = "render")]
use self::render::*;
pub use self::types::*;
pub use self::userdata::*;
pub use self::view::{Positioner, View, WeakView};

use self::view::VIEWS;

pub use input::keyboard::Key;
pub use input::pointer::Button;

struct NotRequiredThreadSafe<T>(T);
unsafe impl<T> Send for NotRequiredThreadSafe<T> {}
unsafe impl<T> Sync for NotRequiredThreadSafe<T> {}

lazy_static! {
    static ref CALLBACK:
        NotRequiredThreadSafe<UnsafeCell<Option<Box<Callback>>>>
        = NotRequiredThreadSafe(UnsafeCell::new(None));
}

extern "C" fn ffi_output_created(handle: ffi::wlc_handle) -> bool {
    match unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        Some(ref mut callback) => callback.output_created(output::from_handle(handle)),
        None => true,
    }
}

extern "C" fn ffi_output_destroyed(handle: ffi::wlc_handle) {
    let output = output::from_handle(handle);
    OUTPUTS.0.borrow_mut().remove(&handle);

    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.output_destroyed(output);
    }

    let output = output::from_handle(handle);
    output.clear_user_data();
}

extern "C" fn ffi_output_focus(handle: ffi::wlc_handle, focus: bool) {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.output_focus(output::from_handle(handle), focus)
    }
}

extern "C" fn ffi_output_resolution(handle: ffi::wlc_handle, from: *const ffi::wlc_size,
                                    to: *const ffi::wlc_size) {
    unsafe {
        if let Some(ref mut callback) = (&mut *CALLBACK.0.get()).as_mut() {
            callback.output_resolution(output::from_handle(handle),
                                       Size::from_ffi(&*from),
                                       Size::from_ffi(&*to))
        }
    }
}

#[cfg(not(feature = "render"))]
extern "C" fn ffi_output_render_pre(handle: ffi::wlc_handle) {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.output_render_pre(output::from_handle(handle))
    }
}

#[cfg(feature = "render")]
extern "C" fn ffi_output_render_pre(handle: ffi::wlc_handle) {
    unsafe {
        if let Some(ref mut callback) = (&mut *CALLBACK.0.get()).as_mut() {
            callback.output_render_pre(mem::transmute(output::from_handle(handle)))
        }
    }
}

#[cfg(not(feature = "render"))]
extern "C" fn ffi_output_render_post(handle: ffi::wlc_handle) {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.output_render_post(output::from_handle(handle))
    }
}

#[cfg(feature = "render")]
extern "C" fn ffi_output_render_post(handle: ffi::wlc_handle) {
    unsafe {
        if let Some(ref mut callback) = (&mut *CALLBACK.0.get()).as_mut() {
            callback.output_render_post(mem::transmute(output::from_handle(handle)))
        }
    }
}

extern "C" fn ffi_output_context_created(handle: ffi::wlc_handle) {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.output_context_created(output::from_handle(handle))
    }
}

extern "C" fn ffi_output_context_destroyed(handle: ffi::wlc_handle) {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.output_context_destroyed(output::from_handle(handle))
    }
}

extern "C" fn ffi_view_created(handle: ffi::wlc_handle) -> bool {
    match unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        Some(ref mut callback) => callback.view_created(view::from_handle(handle)),
        None => true,
    }
}

extern "C" fn ffi_view_destroyed(handle: ffi::wlc_handle) {
    let view = view::from_handle(handle);
    VIEWS.0.borrow_mut().remove(&handle);

    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.view_destroyed(view);
    }

    let view = view::from_handle(handle);
    view.clear_user_data();
}

extern "C" fn ffi_view_focus(handle: ffi::wlc_handle, focus: bool) {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.view_focus(view::from_handle(handle), focus)
    }
}

extern "C" fn ffi_view_move_to_output(handle: ffi::wlc_handle, out1: ffi::wlc_handle, out2: ffi::wlc_handle) {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.view_move_to_output(view::from_handle(handle),
                                     output::from_handle(out1),
                                     output::from_handle(out2))
    }
}

extern "C" fn ffi_view_request_geometry(handle: ffi::wlc_handle, geometry: *const ffi::wlc_geometry) {
    unsafe {
        if let Some(ref mut callback) = (&mut *CALLBACK.0.get()).as_mut() {
            callback.view_request_geometry(view::from_handle(handle), Geometry::from_ffi(&*geometry))
        }
    }
}

extern "C" fn ffi_view_request_state(handle: ffi::wlc_handle, state: ffi::wlc_view_state_bit, toggle: bool) {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.view_request_state(view::from_handle(handle),
                                    ViewState::Flags::from_bits_truncate(state),
                                    toggle)
    }
}

extern "C" fn ffi_view_request_move(handle: ffi::wlc_handle, to: *const ffi::wlc_point) {
    unsafe {
        if let Some(ref mut callback) = (&mut *CALLBACK.0.get()).as_mut() {
            callback.view_request_move(view::from_handle(handle), Point::from_ffi(&*to))
        }
    }
}

extern "C" fn ffi_view_request_resize(handle: ffi::wlc_handle, edges: u32, to: *const ffi::wlc_point) {
    unsafe {
        if let Some(ref mut callback) = (&mut *CALLBACK.0.get()).as_mut() {
            callback.view_request_resize(view::from_handle(handle),
                                         ResizeEdge::Flags::from_bits_truncate(edges),
                                         Point::from_ffi(&*to))
        }
    }
}

#[cfg(not(feature = "render"))]
extern "C" fn ffi_view_render_pre(handle: ffi::wlc_handle) {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.view_render_pre(view::from_handle(handle))
    }
}

#[cfg(feature = "render")]
extern "C" fn ffi_view_render_pre(handle: ffi::wlc_handle) {
    unsafe {
        if let Some(ref mut callback) = (&mut *CALLBACK.0.get()).as_mut() {
            callback.view_render_pre(mem::transmute(view::from_handle(handle)))
        }
    }
}

#[cfg(not(feature = "render"))]
extern "C" fn ffi_view_render_post(handle: ffi::wlc_handle) {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.view_render_post(view::from_handle(handle))
    }
}

#[cfg(feature = "render")]
extern "C" fn ffi_view_render_post(handle: ffi::wlc_handle) {
    unsafe {
        if let Some(ref mut callback) = (&mut *CALLBACK.0.get()).as_mut() {
            callback.view_render_post(mem::transmute(view::from_handle(handle)))
        }
    }
}

extern "C" fn ffi_view_properties_updated(handle: ffi::wlc_handle, mask: u32) {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.view_properties_updated(view::from_handle(handle),
                                         ViewPropertyUpdate::Flags::from_bits_truncate(mask))
    }
}

extern "C" fn ffi_keyboard_key(handle: ffi::wlc_handle, time: u32, mods: *const ffi::wlc_modifiers,
                               key: u32, state: ffi::wlc_key_state)
                               -> bool {
    let view = view::from_handle(handle);

    unsafe {
        match (&mut *CALLBACK.0.get()).as_mut() {
            Some(ref mut callback) => {
                if let Some(key) = Key::from_u32(key) {
                    callback.keyboard_key(if handle == 0 { None } else { Some(view) },
                                          time,
                                          Modifiers::from_ffi(&*mods),
                                          key,
                                          KeyState::from_u32(state)
                                              .expect(&format!("Wlc send an unknown KeyState {}. Aborting",
                                                               state)))
                } else {
                    warn!("Wlc send unknown key: {}. Ignoring", key);
                    false
                }
            }
            None => false,
        }
    }
}

extern "C" fn ffi_pointer_button(handle: ffi::wlc_handle, time: u32, mods: *const ffi::wlc_modifiers,
                                 button: u32, state: ffi::wlc_button_state, at: *const ffi::wlc_point)
                                 -> bool {
    let view = view::from_handle(handle);

    unsafe {
        match (&mut *CALLBACK.0.get()).as_mut() {
            Some(ref mut callback) => {
                if let Some(button) = Button::from_u32(button) {
                    callback.pointer_button(if handle == 0 { None } else { Some(view) },
                                        time,
                                        Modifiers::from_ffi(&*mods),
                                        button,
                                        ButtonState::from_u32(state)
                                            .expect(&format!("Wlc send an unknown ButtonState {}. Aborting",
                                                             state)),
                                        Point::from_ffi(&*at))
                } else {
                    warn!("Wlc send an unknown Button {}. Ignoring", button);
                    false
                }
            }
            None => false,
        }
    }
}

extern "C" fn ffi_pointer_scroll(handle: ffi::wlc_handle, time: u32, mods: *const ffi::wlc_modifiers,
                                 axis_bits: u8, amount: *mut f64)
                                 -> bool {
    use std::slice;

    let safe_amount = unsafe { slice::from_raw_parts(amount, 2) };
    let copy_amount = [safe_amount[0], safe_amount[1]];

    let view = view::from_handle(handle);
    unsafe {
        match (&mut *CALLBACK.0.get()).as_mut() {
            Some(ref mut callback) => {
                callback.pointer_scroll(if handle == 0 { None } else { Some(view) },
                                        time,
                                        Modifiers::from_ffi(&*mods),
                                        ScrollAxis::Flags::from_bits_truncate(axis_bits),
                                        copy_amount)
            }
            None => false,
        }
    }
}

extern "C" fn ffi_pointer_motion(handle: ffi::wlc_handle, time: u32, point: *const ffi::wlc_point) -> bool {
    let view = view::from_handle(handle);

    unsafe {
        match (&mut *CALLBACK.0.get()).as_mut() {
            Some(ref mut callback) => {
                callback.pointer_motion(if handle == 0 { None } else { Some(view) },
                                        time,
                                        Point::from_ffi(&*point))
            }
            None => false,
        }
    }
}

extern "C" fn ffi_touch(handle: ffi::wlc_handle, time: u32, mods: *const ffi::wlc_modifiers,
                        touch: ffi::wlc_touch_type, slot: i32, point: *const ffi::wlc_point)
                        -> bool {
    let view = view::from_handle(handle);

    unsafe {
        match (&mut *CALLBACK.0.get()).as_mut() {
            Some(ref mut callback) => {
                callback.touch(if handle == 0 { None } else { Some(view) },
                               time,
                               Modifiers::from_ffi(&*mods),
                               TouchType::from_u32(touch)
                                   .expect(&format!("Wlc send an unknown TouchType {}. Aborting", touch)),
                               slot,
                               Point::from_ffi(&*point))
            }
            None => false,
        }
    }
}

extern "C" fn ffi_compositor_ready() {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.compositor_ready()
    }
}

extern "C" fn ffi_compositor_terminate() {
    if let Some(ref mut callback) = unsafe { &mut *CALLBACK.0.get() }.as_mut() {
        callback.compositor_terminate()
    }
}

unsafe extern "C" fn ffi_wlc_log_handler(log_type: ffi::wlc_log_type, msg: *const libc::c_char) {
    let msg = CStr::from_ptr(msg);
    match log_type {
        ffi::wlc_log_type_WLC_LOG_ERROR => error!("{:?}", msg),
        ffi::wlc_log_type_WLC_LOG_WARN => warn!("{:?}", msg),
        ffi::wlc_log_type_WLC_LOG_INFO => info!("{:?}", msg),
        ffi::wlc_log_type_WLC_LOG_WAYLAND => debug!("{:?}", msg),
        _ => unreachable!(),
    };
}

/// Entry point for your code
///
/// Implementing `Callback` and passing it to `wlc::init` allows you to interact
/// with wlc. The compositor will call you, when any events happens with the
/// appropriate function.
#[allow(unused_variables)]
pub trait Callback {
    /// Output was created.
    ///
    /// Return `false` if you want to destroy the output
    /// (e.g. failed to allocate data related to output)
    ///
    /// # Note
    /// This is not necessarily the first function called on a new output. No
    /// order is guaranteed
    fn output_created(&mut self, output: &Output) -> bool {
        true
    }

    /// Output was destroyed
    fn output_destroyed(&mut self, output: &Output) {}

    /// Output got or lost focus
    fn output_focus(&mut self, output: &Output, focus: bool) {}

    /// Output resolution changed
    fn output_resolution(&mut self, output: &Output, from: Size, to: Size) {}

    /// Output pre render hook
    #[cfg(not(feature = "render"))]
    fn output_render_pre(&mut self, output: &Output) {}

    /// Output pre render hook
    #[cfg(feature = "render")]
    fn output_render_pre(&mut self, output: &mut RenderOutput) {}

    /// Output post render hook
    #[cfg(not(feature = "render"))]
    fn output_render_post(&mut self, output: &Output) {}

    /// Output post render hook
    #[cfg(feature = "render")]
    fn output_render_post(&mut self, output: &mut RenderOutput) {}

    /// Output context is created.
    ///
    /// This generally happens on startup and when current tty changes
    fn output_context_created(&mut self, output: &Output) {}

    /// Output context was destroyed
    fn output_context_destroyed(&mut self, output: &Output) {}

    /// View was created.
    ///
    /// Return `false` if you want to destroy the view
    /// (e.g. failed to allocate data related to view)
    ///
    /// # Note
    /// This is not necessarily the first function called on a new view. No
    /// order is guaranteed
    fn view_created(&mut self, view: &View) -> bool {
        true
    }

    /// View was destroyed
    fn view_destroyed(&mut self, view: &View) {}

    /// View got or lost focus
    fn view_focus(&mut self, view: &View, focus: bool) {}

    /// View was moved to output
    fn view_move_to_output(&mut self, view: &View, from: &Output, to: &Output) {}

    /// Request to set given geometry for view.
    ///
    /// Apply using `view.set_geometry` to agree
    fn view_request_geometry(&mut self, view: &View, geometry: Geometry) {}

    /// Request to disable or enable the given state for view.
    ///
    /// Apply using `view.set_state` to agree
    fn view_request_state(&mut self, view: &View, state: ViewState::Flags, toggle: bool) {}

    /// Request to move itself.
    ///
    /// Start an interactive move to agree
    fn view_request_move(&mut self, view: &View, origin: Point) {}

    /// Request to resize itself with the given edges.
    ///
    /// Start a interactive resize to agree
    fn view_request_resize(&mut self, view: &View, edges: ResizeEdge::Flags, origin: Point) {}

    /// View pre render hook
    #[cfg(not(feature = "render"))]
    fn view_render_pre(&mut self, view: &View) {}

    /// View pre render hook
    #[cfg(feature = "render")]
    fn view_render_pre(&mut self, view: &mut RenderView) {}

    /// View post render hook
    #[cfg(not(feature = "render"))]
    fn view_render_post(&mut self, view: &View) {}

    /// View post render hook
    #[cfg(feature = "render")]
    fn view_render_post(&mut self, view: &mut RenderView) {}

    /// View properties (title, class, app_id) were updated
    fn view_properties_updated(&mut self, view: &View, mask: ViewPropertyUpdate::Flags) {}

    /// Key event was triggered, view handle will be zero if there was no focus.
    ///
    /// Return true to prevent sending the event to clients
    fn keyboard_key(&mut self, view: Option<&View>, time: u32, modifiers: Modifiers, key: Key,
                    state: KeyState)
                    -> bool {
        false
    }

    /// Button event was triggered, view handle will be `None` if there was no
    /// focus.
    ///
    /// Return true to prevent sending the event to clients
    fn pointer_button(&mut self, view: Option<&View>, time: u32, modifiers: Modifiers, button: Button,
                      state: ButtonState, origin: Point)
                      -> bool {
        false
    }

    /// Scroll event was triggered, view handle will be `None` if there was no
    /// focus.
    ///
    /// Return true to prevent sending the event to clients
    fn pointer_scroll(&mut self, view: Option<&View>, time: u32, modifiers: Modifiers,
                      axis: ScrollAxis::Flags, amount: [f64; 2])
                      -> bool {
        false
    }

    /// Motion event was triggered, view handle will be `None` if there was no
    /// focus.
    ///
    /// Apply with `pointer::set_position()` to agree.
    /// Return `true` to prevent sending the event to clients
    fn pointer_motion(&mut self, view: Option<&View>, time: u32, origin: Point) -> bool {
        false
    }

    /// Touch event was triggered, view handle will be `None` if there was no
    /// focus.
    ///
    /// Return `true` to prevent sending the event to clients
    fn touch(&mut self, view: Option<&View>, time: u32, modifiers: Modifiers, touch_type: TouchType,
             slot: i32, origin: Point)
             -> bool {
        false
    }

    /// Compositor is ready to accept clients
    fn compositor_ready(&mut self) {}

    /// Compositor is about to terminate
    fn compositor_terminate(&mut self) {}

    // Input device was created. Return value does nothing. (Experimental)
    // fn input_created(&mut self, device: LipInputDevice) -> bool { true }

    // Input device was destroyed. (Experimental)
    // fn input_destroyed(&mut self, device: LipInputDevice) {}
}

impl<C: Callback + ?Sized> Callback for Box<C> {
    fn output_created(&mut self, output: &Output) -> bool {
        (**self).output_created(output)
    }
    fn output_destroyed(&mut self, output: &Output) {
        (**self).output_destroyed(output)
    }
    fn output_focus(&mut self, output: &Output, focus: bool) {
        (**self).output_focus(output, focus)
    }
    fn output_resolution(&mut self, output: &Output, from: Size, to: Size) {
        (**self).output_resolution(output, from, to)
    }
    #[cfg(not(feature = "render"))]
    fn output_render_pre(&mut self, output: &Output) {
        (**self).output_render_pre(output)
    }
    #[cfg(feature = "render")]
    fn output_render_pre(&mut self, output: &mut RenderOutput) {
        (**self).output_render_pre(output)
    }
    #[cfg(not(feature = "render"))]
    fn output_render_post(&mut self, output: &Output) {
        (**self).output_render_post(output)
    }
    #[cfg(feature = "render")]
    fn output_render_post(&mut self, output: &mut RenderOutput) {
        (**self).output_render_post(output)
    }
    fn output_context_created(&mut self, output: &Output) {
        (**self).output_context_created(output)
    }
    fn output_context_destroyed(&mut self, output: &Output) {
        (**self).output_context_destroyed(output)
    }
    fn view_created(&mut self, view: &View) -> bool {
        (**self).view_created(view)
    }
    fn view_destroyed(&mut self, view: &View) {
        (**self).view_destroyed(view)
    }
    fn view_focus(&mut self, view: &View, focus: bool) {
        (**self).view_focus(view, focus)
    }
    fn view_move_to_output(&mut self, view: &View, from: &Output, to: &Output) {
        (**self).view_move_to_output(view, from, to)
    }
    fn view_request_geometry(&mut self, view: &View, geometry: Geometry) {
        (**self).view_request_geometry(view, geometry)
    }
    fn view_request_state(&mut self, view: &View, state: ViewState::Flags, toggle: bool) {
        (**self).view_request_state(view, state, toggle)
    }
    fn view_request_move(&mut self, view: &View, origin: Point) {
        (**self).view_request_move(view, origin)
    }
    fn view_request_resize(&mut self, view: &View, edges: ResizeEdge::Flags, origin: Point) {
        (**self).view_request_resize(view, edges, origin)
    }
    #[cfg(not(feature = "render"))]
    fn view_render_pre(&mut self, view: &View) {
        (**self).view_render_pre(view)
    }
    #[cfg(feature = "render")]
    fn view_render_pre(&mut self, view: &mut RenderView) {
        (**self).view_render_pre(view)
    }
    #[cfg(not(feature = "render"))]
    fn view_render_post(&mut self, view: &View) {
        (**self).view_render_post(view)
    }
    #[cfg(feature = "render")]
    fn view_render_post(&mut self, view: &mut RenderView) {
        (**self).view_render_post(view)
    }
    fn view_properties_updated(&mut self, view: &View, mask: ViewPropertyUpdate::Flags) {
        (**self).view_properties_updated(view, mask)
    }
    fn keyboard_key(&mut self, view: Option<&View>, time: u32, modifiers: Modifiers, key: Key,
                    state: KeyState)
                    -> bool {
        (**self).keyboard_key(view, time, modifiers, key, state)
    }
    fn pointer_button(&mut self, view: Option<&View>, time: u32, modifiers: Modifiers, button: Button,
                      state: ButtonState, origin: Point)
                      -> bool {
        (**self).pointer_button(view, time, modifiers, button, state, origin)
    }
    fn pointer_scroll(&mut self, view: Option<&View>, time: u32, modifiers: Modifiers,
                      axis: ScrollAxis::Flags, amount: [f64; 2])
                      -> bool {
        (**self).pointer_scroll(view, time, modifiers, axis, amount)
    }
    fn pointer_motion(&mut self, view: Option<&View>, time: u32, origin: Point) -> bool {
        (**self).pointer_motion(view, time, origin)
    }
    fn touch(&mut self, view: Option<&View>, time: u32, modifiers: Modifiers, touch_type: TouchType,
             slot: i32, origin: Point)
             -> bool {
        (**self).touch(view, time, modifiers, touch_type, slot, origin)
    }
    fn compositor_ready(&mut self) {
        (**self).compositor_ready()
    }
    fn compositor_terminate(&mut self) {
        (**self).compositor_terminate()
    }
}

impl Callback for () {}

/// Initialized compositor
///
/// Dropping this will start wlc and block the current thread until it
/// terminates
pub struct Wlc;

#[cfg(not(feature = "unsafe-stable"))]
impl !Sync for Wlc {}
#[cfg(not(feature = "unsafe-stable"))]
impl !Send for Wlc {}

impl Drop for Wlc {
    fn drop(&mut self) {
        unsafe {
            ffi::wlc_run();
        }
    }
}

/// Error representing the failure to start the Compositor
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Error {
    /// A wlc compositor was already started in this library and did not yet
    /// terminate
    AlreadyRunning,
    /// wlc encountered an internal error (e.g. failure to allocate resources)
    /// Take a log at the log (wlc uses the [log](https://crates.io/crates/log)
    /// crate)
    InternalError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error starting Wlc: {}", self.description())
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AlreadyRunning => "An instance of Wlc is already running",
            Error::InternalError => "Wlc encountered an internal Error",
        }
    }
}

/// Initialize Wlc. Returns `wlc::Error` on failure.
///
/// # Warning
/// Avoid running unverified code before `wlc::init` as wlc compositor may be
/// run with
/// higher privileges on non logind systems where compositor binary needs to be
/// suid.
/// `wlc::init`'s purpose is to initialize and drop privileges as soon as
/// possible.
///
/// # Safety
/// Dont call this function on another thread, then the main thread.
/// The whole library is bound to the main thread and neither `Send` or `Sync` !
///
/// # Notes
/// - The Callbacks may not be changed later on
/// - Dropping the return value - letting it go out of scope - starts the
/// actual compositor in a blocking fashion.
pub fn init<T: Callback + 'static>(callbacks: T) -> Result<Wlc, Error> {
    if unsafe { &*CALLBACK.0.get() }.is_some() {
        return Err(Error::AlreadyRunning);
    }

    unsafe {
        ffi::wlc_log_set_handler(Some(ffi_wlc_log_handler));
    }

    unsafe {
        ffi::wlc_set_output_created_cb(Some(ffi_output_created));
        ffi::wlc_set_output_destroyed_cb(Some(ffi_output_destroyed));
        ffi::wlc_set_output_focus_cb(Some(ffi_output_focus));
        ffi::wlc_set_output_resolution_cb(Some(ffi_output_resolution));
        ffi::wlc_set_output_render_pre_cb(Some(ffi_output_render_pre));
        ffi::wlc_set_output_render_post_cb(Some(ffi_output_render_post));
        ffi::wlc_set_output_context_created_cb(Some(ffi_output_context_created));
        ffi::wlc_set_output_context_destroyed_cb(Some(ffi_output_context_destroyed));
        ffi::wlc_set_view_created_cb(Some(ffi_view_created));
        ffi::wlc_set_view_destroyed_cb(Some(ffi_view_destroyed));
        ffi::wlc_set_view_focus_cb(Some(ffi_view_focus));
        ffi::wlc_set_view_move_to_output_cb(Some(ffi_view_move_to_output));
        ffi::wlc_set_view_request_geometry_cb(Some(ffi_view_request_geometry));
        ffi::wlc_set_view_request_state_cb(Some(ffi_view_request_state));
        ffi::wlc_set_view_request_move_cb(Some(ffi_view_request_move));
        ffi::wlc_set_view_request_resize_cb(Some(ffi_view_request_resize));
        ffi::wlc_set_view_render_pre_cb(Some(ffi_view_render_pre));
        ffi::wlc_set_view_render_post_cb(Some(ffi_view_render_post));
        ffi::wlc_set_view_properties_updated_cb(Some(ffi_view_properties_updated));
        ffi::wlc_set_keyboard_key_cb(Some(ffi_keyboard_key));
        ffi::wlc_set_pointer_button_cb(Some(ffi_pointer_button));
        ffi::wlc_set_pointer_scroll_cb(Some(ffi_pointer_scroll));
        ffi::wlc_set_pointer_motion_cb(Some(ffi_pointer_motion));
        ffi::wlc_set_touch_cb(Some(ffi_touch));
        ffi::wlc_set_compositor_ready_cb(Some(ffi_compositor_ready));
        ffi::wlc_set_compositor_terminate_cb(Some(ffi_compositor_terminate));
    }

    mem::swap(&mut Some(Box::new(callbacks) as Box<Callback>),
              unsafe { &mut *CALLBACK.0.get() });

    if unsafe { ffi::wlc_init() } {
        Ok(Wlc)
    } else {
        Err(Error::InternalError)
    }
}

/// Terminates the currently active wlc Compositor
///
/// # Safety
/// Dont call this function on another thread, then the main thread
pub fn terminate() {
    unsafe { ffi::wlc_terminate() }
}

/// Gets the currently active Backend
///
/// # Safety
/// Dont call this function on another thread, then the main thread!
pub fn get_backend_type() -> BackendType {
    BackendType::from_u32(unsafe { ffi::wlc_get_backend_type() })
        .expect("WLC send an unknown BackendType. Aborting")
}
