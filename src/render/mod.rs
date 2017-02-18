//! wlc's Rendering extensions
//!
//! Enabled by feature `render`
//!
//! Some of the original function may be added as functions to View and Output,
//! when this feature is enabled
//!
//! The types/functions in this module provide some basic rendering
//! capabilities.
//!
//! For more advanced drawing you should directly use GLES2.
//! This is not documented as it's currently relying on the implementation
//! details of wlc.

use Geometry;

use ffi;
use libc;

use std::marker::PhantomData;
use std::mem;

#[cfg(feature = "wayland")]
use wayland::WlcSurface;

/// Wlc's Renderer Types
pub enum RenderInstance<'a> {
    /// Renderer using GLES2
    GLES2(GLES2Renderer<'a>),
    /// Headless Renderer
    None(NoRenderer<'a>),
}

/// Renderer specific pixel format marker
pub trait PixelFormat {
    #[doc(hidden)]
    fn as_bits(&self) -> u32;
}

/// Renderer specific Texture marker
pub trait Texture {}

/// Renderer API
pub trait Renderer {
    /// PixelFormat understood by this Renderer
    type PixelFormat: PixelFormat;
    /// Texture understood by this Renderer
    type Texture: Texture;

    /// Write pixel data with the specific format to output's framebuffer.
    ///
    /// If the geometry is out of bounds, it will be automaticall clamped.
    fn pixels_write(&mut self, format: Self::PixelFormat, geometry: Geometry, data: &[u8]);

    /// Read pixel data from output's framebuffer.
    ///
    /// If the geometry is out of bounds, it will be automatically clamped.
    /// Potentially clamped geometry will change geometry, to indicate width /
    /// height of the returned data.
    fn pixels_read(&self, format: Self::PixelFormat, geometry: &mut Geometry) -> Vec<u8>;

    /// Renders surface
    #[cfg(feature = "wayland")]
    fn render_surface(&mut self, surface: &mut WlcSurface, geometry: Geometry);

    /// Returns the textures of a surface. Returns an Err if surface is invalid.
    #[cfg(feature = "wayland")]
    fn surface_textures(&self, surface: &mut WlcSurface) -> Result<([Self::Texture; 3], SurfaceFormat), ()>;
}

enum_from_primitive! {
    #[derive(Clone, Copy, Debug)]
    #[repr(u32)]
    #[allow(non_camel_case_types, missing_docs)]
    /// Possible Surface formats
    pub enum SurfaceFormat {
        RGB     = ffi::wlc_surface_format_SURFACE_RGB,
        RGBA    = ffi::wlc_surface_format_SURFACE_RGBA,
        EGL     = ffi::wlc_surface_format_SURFACE_EGL,
        Y_UV    = ffi::wlc_surface_format_SURFACE_Y_UV,
        Y_U_V   = ffi::wlc_surface_format_SURFACE_Y_U_V,
        Y_XUXV  = ffi::wlc_surface_format_SURFACE_Y_XUXV,
        None,
    }
}

/// `PixelFormat` of the `NoRenderer`
pub struct NoPixelFormat;
impl PixelFormat for NoPixelFormat {
    fn as_bits(&self) -> u32 {
        0
    }
}

/// Texture of the `NoRenderer`
pub struct NoTexture;
impl Texture for NoTexture {}

enum_from_primitive! {
    #[derive(Clone, Copy, Debug)]
    #[repr(u32)]
    #[allow(missing_docs)]
    /// Supported PixelFormats of the GLES2Renderer
    pub enum GLES2PixelFormat {
        RGBA8888 = ffi::wlc_pixel_format_WLC_RGBA8888,
    }
}

impl PixelFormat for GLES2PixelFormat {
    fn as_bits(&self) -> u32 {
        *self as u32
    }
}

/// Texture of the `GLES2Renderer` (Raw OpenGL ES 2 Texture)
pub type GLTexture = libc::c_uint;
impl Texture for GLTexture {}

/// Headless renderer
pub struct NoRenderer<'a>(PhantomData<&'a ()>);

impl<'a> Renderer for NoRenderer<'a> {
    type PixelFormat = NoPixelFormat;
    type Texture = NoTexture;

    fn pixels_write(&mut self, _format: NoPixelFormat, _geometry: Geometry, _data: &[u8]) {}

    fn pixels_read(&self, _format: NoPixelFormat, _geometry: &mut Geometry) -> Vec<u8> {
        Vec::new()
    }

    #[cfg(feature = "wayland")]
    fn render_surface(&mut self, _surface: &mut WlcSurface, _geometry: Geometry) {}

    #[cfg(feature = "wayland")]
    fn surface_textures(&self, _surface: &mut WlcSurface) -> Result<([NoTexture; 3], SurfaceFormat), ()> {
        Ok(([NoTexture, NoTexture, NoTexture], SurfaceFormat::None))
    }
}

/// OpenGL ES 2 Renderer
pub struct GLES2Renderer<'a>(PhantomData<&'a ()>);
#[cfg(not(feature = "unsafe-stable"))]
impl<'a> !Sync for GLES2Renderer<'a> {}
#[cfg(not(feature = "unsafe-stable"))]
impl<'a> !Send for GLES2Renderer<'a> {}

impl<'a> Renderer for GLES2Renderer<'a> {
    type PixelFormat = GLES2PixelFormat;
    type Texture = GLTexture;

    fn pixels_write(&mut self, format: GLES2PixelFormat, geometry: Geometry, data: &[u8]) {
        unsafe {
            ffi::wlc_pixels_write(format as u32,
                                  &geometry.into_ffi() as *const _,
                                  data.as_ptr() as *const _)
        }
    }

    fn pixels_read(&self, format: GLES2PixelFormat, geometry: &mut Geometry) -> Vec<u8> {
        let size = geometry.size.w * geometry.size.h;
        let mut data = Vec::with_capacity(size as usize * 4);
        unsafe {
            data.set_len(size as usize * 4);
            let mut final_geometry: ffi::wlc_geometry = mem::uninitialized();
            ffi::wlc_pixels_read(format as u32,
                                 &geometry.into_ffi() as *const _,
                                 &mut final_geometry as *mut _,
                                 data.as_mut_slice().as_mut_ptr() as *mut _);
            *geometry = Geometry::from_ffi(&final_geometry);

            data.truncate((geometry.size.w * geometry.size.h) as usize * 4);

            data
        }

    }

    #[cfg(feature = "wayland")]
    fn render_surface(&mut self, surface: &mut WlcSurface, geometry: Geometry) {
        unsafe {
            ffi::wlc_surface_render(*mem::transmute::<&mut WlcSurface, &mut libc::uintptr_t>(surface),
                                    &geometry.into_ffi() as *const _)
        }
    }

    #[cfg(feature = "wayland")]
    fn surface_textures(&self, surface: &mut WlcSurface) -> Result<([GLTexture; 3], SurfaceFormat), ()> {
        unsafe {
            let mut textures: [GLTexture; 3] = mem::uninitialized();
            let mut format: SurfaceFormat = mem::uninitialized();

            if ffi::wlc_surface_get_textures(*mem::transmute::<&mut WlcSurface,
                                                               &mut libc::uintptr_t>(surface),
                                             mem::transmute(&mut textures as *mut [u32; 3]),
                                             mem::transmute::<&mut SurfaceFormat, &mut u32>(&mut format) as
                                             *mut _) {
                Ok((textures, format))
            } else {
                Err(())
            }
        }
    }
}

mod output;
mod view;

pub use self::output::*;
pub use self::view::*;
