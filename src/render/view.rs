use super::RenderOutput;
use View;

use ffi;

use std::fmt;
use std::mem;
use std::ops::Deref;

/// View with Render Extensions
///
/// Used by the `Callback::view_render_pre` and `Callback::view_render_post`
/// events
#[repr(C)]
pub struct RenderView;

impl Deref for RenderView {
    type Target = View;

    fn deref(&self) -> &View {
        unsafe { mem::transmute(self) }
    }
}

impl fmt::Debug for RenderView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", **self)
    }
}

impl RenderView {
    /// Receive current render-extensions enabled `Output`
    pub fn output(&mut self) -> &mut RenderOutput {
        unsafe { &mut *(ffi::wlc_view_get_output(mem::transmute(self)) as *mut RenderOutput) }
    }
}
