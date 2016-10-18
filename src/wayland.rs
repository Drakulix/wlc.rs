//! wlc's Wayland extensions
//!
//! Enabled by feature `wayland`
//!
//! Some of the original function may be directly added as functions to View
//! and Output,
//! when this feature is enabled.

use {Geometry, Output, Size, View, WeakOutput, WeakView};

use ffi;

use std::mem;
use std::slice;

use wayland_server::Resource;
use wayland_server::protocol::wl_output::WlOutput;
use wayland_server::protocol::wl_surface::WlSurface;

#[allow(missing_docs)]
/// The following wlc function are exposed here for own implementation
/// as a general purpose implementation is out of scope of this crate.
pub mod sys {
    pub use ffi::wlc_get_wl_display;
    pub use ffi::wlc_view_from_surface;
    pub use ffi::wlc_view_get_role;
}

/// Returns a weak view handle from `WlSurface` resource
pub fn view_from_wl_surface(surface: &WlSurface) -> WeakView {
    let view: &View =
        unsafe { &*(ffi::wlc_handle_from_wl_surface_resource(surface.ptr() as *mut _) as *const View) };

    view.weak_reference()
}

/// Returns a weak output handle from `WlOutput` resource
pub fn output_from_wl_surface(output: &WlOutput) -> WeakOutput {
    let output: &Output =
        unsafe { &*(ffi::wlc_handle_from_wl_output_resource(output.ptr() as *mut _) as *const Output) };

    output.weak_reference()
}

/// Internal Surface Struct
#[repr(C)]
#[derive(Clone, Debug)]
pub struct WlcSurface;
#[cfg(not(feature = "unsafe-stable"))]
impl !Sync for WlcSurface {}
#[cfg(not(feature = "unsafe-stable"))]
impl !Send for WlcSurface {}

/// Returns internal wlc surface from `WlSurface` resource
pub fn wlc_resource_from_wl_surface(surface: &WlSurface) -> &WlcSurface {
    unsafe { &*(ffi::wlc_resource_from_wl_surface_resource(surface.ptr() as *mut _) as *const WlcSurface) }
}

impl WlcSurface {
    /// Get surface size
    pub fn size(&self) -> Size {
        unsafe { Size::from_ffi(&*ffi::wlc_surface_get_size(handle(self))) }
    }

    /// Return wl_surface resource from internal wlc surface
    pub fn wl_surface(&self) -> WlSurface {
        unsafe { WlSurface::from_ptr_new(ffi::wlc_surface_get_wl_resource(handle(self)) as *mut _) }
    }

    /// Returns a list of the subsurfaces of the given surface
    pub fn sub_surfaces(&self) -> &[&WlcSubSurface] {
        unsafe {
            let mut size = 0;
            let ptr = ffi::wlc_surface_get_subsurfaces(handle(self), &mut size as *mut _) as *const _;
            slice::from_raw_parts(ptr, size)
        }
    }

    /// Adds frame callbacks of the given surface for the next output frame.
    /// It applies recursively to all subsurfaces.
    ///
    /// Useful when the compositor creates custom animations which require
    /// disabling internal rendering,
    /// but still need to update the surface textures (for ex. video players).
    #[cfg(feature = "render")]
    pub fn flush_frame_callbacks(&self) {
        unsafe { ffi::wlc_surface_flush_frame_callbacks(handle(self)) }
    }
}

fn handle(surface: &WlcSurface) -> ffi::wlc_resource {
    unsafe { mem::transmute(surface) }
}

/// Subsurface of an `WlcSurface`
#[repr(C)]
pub struct WlcSubSurface;
#[cfg(not(feature = "unsafe-stable"))]
impl !Sync for WlcSubSurface {}
#[cfg(not(feature = "unsafe-stable"))]
impl !Send for WlcSubSurface {}

impl WlcSubSurface {
    /// Returns the size of a subsurface and its position relative to parent
    pub fn geometry(&self) -> Geometry {
        unsafe {
            let mut geo: ffi::wlc_geometry = mem::uninitialized();
            ffi::wlc_get_subsurface_geometry(mem::transmute(self), &mut geo as *mut _);
            Geometry::from_ffi(&geo)
        }
    }
}
