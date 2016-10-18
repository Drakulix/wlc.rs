use super::{GLES2Renderer, NoRenderer, RenderInstance};
use Output;

use ffi;

use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;

/// Output with Render Extensions
///
/// Used by the `Callback::output_render_pre` and
/// `Callback::output_render_post` events
#[repr(C)]
pub struct RenderOutput;

impl Deref for RenderOutput {
    type Target = Output;

    fn deref(&self) -> &Output {
        unsafe { mem::transmute(self) }
    }
}

impl fmt::Debug for RenderOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", **self)
    }
}

impl RenderOutput {
    /// Returns currently active renderer on the given output
    pub fn get_renderer(&mut self) -> RenderInstance {
        unsafe {
            match ffi::wlc_output_get_renderer(mem::transmute(self)) {
                ffi::wlc_renderer_WLC_NO_RENDERER => RenderInstance::None(NoRenderer(PhantomData)),
                ffi::wlc_renderer_WLC_RENDERER_GLES2 => RenderInstance::GLES2(GLES2Renderer(PhantomData)),
                _ => unreachable!(),
            }
        }
    }
}
