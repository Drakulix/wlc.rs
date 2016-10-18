use {Anchor, ConstraintAdjustment, Geometry, Gravity, Output, Point, ResizeEdge, Size, ViewState, ViewType,
     Visibility};

use ffi;
use libc;

use std::borrow::Cow;
use std::cell::RefCell;
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ptr;
use std::rc::{Rc, Weak};

#[cfg(feature = "wayland")]
use wayland::WlcSurface;
#[cfg(feature = "wayland")]
use wayland_server::Client;

pub struct NotRequiredThreadSafe<T>(pub T);
unsafe impl<T> Send for NotRequiredThreadSafe<T> {}
unsafe impl<T> Sync for NotRequiredThreadSafe<T> {}

lazy_static! {
    pub static ref VIEWS:
        NotRequiredThreadSafe<RefCell<HashMap<libc::uintptr_t, Rc<()>>>> =
        NotRequiredThreadSafe(RefCell::new(HashMap::new()));
}

/// A View managed by Wlc
#[repr(C)]
pub struct View;

#[cfg(not(feature = "unsafe-stable"))]
impl !Sync for View {}
#[cfg(not(feature = "unsafe-stable"))]
impl !Send for View {}

impl fmt::Debug for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "View {{ id: {} }}", handle(self))
    }
}

impl View {
    /// Set no view focused
    ///
    /// # Safety
    /// Dont call this function on another thread, then the main thread
    pub fn set_no_focus() {
        unsafe { ffi::wlc_view_focus(mem::transmute::<*const libc::c_void, libc::uintptr_t>(ptr::null())) }
    }

    /// Focus view
    pub fn focus(&self) {
        unsafe { ffi::wlc_view_focus(handle(self)) }
    }

    /// Close view
    pub fn close(&self) {
        unsafe { ffi::wlc_view_close(handle(self)) }
    }

    /// Get current `Output`
    pub fn output(&self) -> &Output {
        unsafe { &*(ffi::wlc_view_get_output(handle(self)) as *const Output) }
    }

    /// Set `Output`
    pub fn set_output(&self, output: &Output) {
        unsafe {
            ffi::wlc_view_set_output(handle(self),
                                     mem::transmute::<&Output, libc::uintptr_t>(output))
        }
    }

    /// Send behind everything else
    pub fn send_to_back(&self) {
        unsafe { ffi::wlc_view_send_to_back(handle(self)) }
    }

    /// Send below another view
    pub fn send_below(&self, other: &View) {
        unsafe { ffi::wlc_view_send_below(handle(self), handle(other)) }
    }

    /// Send above another view
    pub fn bring_to_front(&self) {
        unsafe { ffi::wlc_view_bring_to_front(handle(self)) }
    }

    /// Bring to front of everything
    pub fn bring_above(&self, other: &View) {
        unsafe { ffi::wlc_view_bring_above(handle(self), handle(other)) }
    }

    /// Get current visibility bitmask
    pub fn visibility(&self) -> Visibility::Flags {
        Visibility::Flags::from_bits_truncate(unsafe { ffi::wlc_view_get_mask(handle(self)) })
    }

    /// Set visibility bitmask
    pub fn set_visibility(&self, visibility: Visibility::Flags) {
        unsafe { ffi::wlc_view_set_mask(handle(self), visibility.bits()) }
    }

    /// Get current geometry. (what client sees)
    pub fn geometry(&self) -> Geometry {
        unsafe { Geometry::from_ffi(&*ffi::wlc_view_get_geometry(handle(self))) }
    }

    /// Get visible geometry. (what wlc displays)
    pub fn visible_geometry(&self) -> Geometry {
        let mut geo: ffi::wlc_geometry = unsafe { mem::uninitialized() };
        unsafe {
            ffi::wlc_view_get_visible_geometry(handle(self), &mut geo as *mut _);
        }
        Geometry::from_ffi(&geo)
    }

    /// Set geometry. Set edges if the geometry change is caused by interactive
    /// resize
    pub fn set_geometry(&self, edge: ResizeEdge::Flags, geometry: Geometry) {
        unsafe { ffi::wlc_view_set_geometry(handle(self), edge.bits(), &geometry.into_ffi() as *const _) }
    }

    /// Get type bitfield
    pub fn view_type(&self) -> ViewType::Flags {
        ViewType::Flags::from_bits_truncate(unsafe { ffi::wlc_view_get_type(handle(self)) })
    }

    /// Set type bit. Toggle indicates whether it is set or not
    pub fn set_view_type(&self, view_type: ViewType::Flags, toggle: bool) {
        unsafe { ffi::wlc_view_set_type(handle(self), view_type.bits(), toggle) }
    }

    /// Get current state bitfield
    pub fn state(&self) -> ViewState::Flags {
        ViewState::Flags::from_bits_truncate(unsafe { ffi::wlc_view_get_state(handle(self)) })
    }

    /// Set state bit. Toggle indicates whether it is set or not
    pub fn set_state(&self, state: ViewState::Flags, toggle: bool) {
        unsafe { ffi::wlc_view_set_state(handle(self), state.bits(), toggle) }
    }

    /// Get parent view
    pub fn parent(&self) -> Option<&View> {
        unsafe {
            let handle = ffi::wlc_view_get_parent(handle(self));
            if handle == 0 {
                None
            } else {
                Some(&*(handle as *mut View))
            }
        }
    }

    /// Set parent view
    pub fn set_parent(&self, parent: &View) {
        unsafe { ffi::wlc_view_set_parent(handle(self), handle(parent)) }
    }

    /// Get title
    pub fn title(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(ffi::wlc_view_get_title(handle(self))).to_string_lossy() }
    }

    /// Get class. (shell-surface only)
    pub fn class(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(ffi::wlc_view_get_class(handle(self))).to_string_lossy() }
    }

    /// Get instance. (shell-surface only)
    pub fn instance(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(ffi::wlc_view_get_instance(handle(self))).to_string_lossy() }
    }

    /// Get app id. (xdg-surface only)
    pub fn app_id(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(ffi::wlc_view_get_app_id(handle(self))).to_string_lossy() }
    }

    /// Get pid
    pub fn pid(&self) -> libc::pid_t {
        unsafe { ffi::wlc_view_get_pid(handle(self)) }
    }

    /// Get the positioner of the `View` if one exists
    ///
    /// The `Positioner` provides a collection of rules for the placement of a
    /// child `View` relative to it's parent.
    /// Compositors are expected to value these hints, but technically not
    /// required.
    pub fn positioner(&self) -> Option<&Positioner> {
        unsafe {
            if ffi::wlc_view_positioner_get_size(handle(self)).is_null() {
                None
            } else {
                Some(mem::transmute::<&View, &Positioner>(self))
            }
        }
    }

    /// Returns internal wlc surface from wl_surface resource
    #[cfg(feature = "wayland")]
    pub fn wl_surface(&self) -> &WlcSurface {
        unsafe { &*(ffi::wlc_view_get_surface(handle(self)) as *const WlcSurface) }
    }

    /// Returns wl_client from view handle
    #[cfg(feature = "wayland")]
    pub fn wl_client(&self) -> Client {
        unsafe { Client::from_ptr(ffi::wlc_view_get_wl_client(handle(self)) as *mut _) }
    }

    /// Get a weak reference of the View that may outlive its referenced view
    ///
    /// Since View is always limited in its use by its lifetime, it is not very
    /// suitable for storing.
    /// This function allows you to optain a weak reference, that may outlive
    /// the view it is referencing.
    pub fn weak_reference(&self) -> WeakView {
        let mut views = VIEWS.0.borrow_mut();
        let ref_counter = views.entry(handle(self)).or_insert_with(|| Rc::new(()));
        WeakView(Rc::downgrade(ref_counter), handle(self))
    }
}

/// A Positioner of a `View` managed by Wlc
#[repr(C)]
pub struct Positioner;

#[cfg(not(feature = "unsafe-stable"))]
impl !Sync for Positioner {}
#[cfg(not(feature = "unsafe-stable"))]
impl !Send for Positioner {}

impl fmt::Debug for Positioner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Positioner of View {{ id: {} }}", handle_pos(self))
    }
}

impl Positioner {
    /// Get the anchor of the `Positioner`
    ///
    /// The anchor defines a set of edges for the anchor rectangle. These are
    /// used to derive an anchor point that the child view will be positioned
    /// relative to.
    /// If two orthogonal edges are specified (e.g. 'top' and 'left'), then the
    /// anchor point will be the intersection of the edges (e.g. the top left
    /// position of the rectangle); otherwise, the derived anchor point will be
    /// centered on the specified edge, or in the center of the anchor
    /// rectangle if no edge is specified.
    pub fn anchor(&self) -> Anchor::Flags {
        Anchor::Flags::from_bits_truncate(unsafe { ffi::wlc_view_positioner_get_anchor(handle_pos(self)) })
    }

    /// Get the anchor rect of the `Positioner`
    ///
    /// Specify the anchor rectangle within the parent's view that the child
    /// will be placed relative to. The rectangle is relative to the window
    /// geometry.
    /// The rectangle will be at least 1x1 large.
    pub fn anchor_rect(&self) -> Geometry {
        unsafe { Geometry::from_ffi(&*ffi::wlc_view_positioner_get_anchor_rect(handle_pos(self))) }
    }

    /// Get the constraint adjustments of the `Positioner`
    ///
    /// The constraint adjustment value defines ways the compositor will adjust
    /// the position of the surface, if the unadjusted position would result in
    /// the surface being partly constrained.
    ///
    /// Whether a surface is considered 'constrained' is left to the compositor
    /// to determine. For example, the surface may be partly outside the
    /// compositor's defined 'work area', thus necessitating the child
    /// surface's position be adjusted until it is entirely inside the work
    /// area.
    ///
    /// The adjustments can be combined, according to a defined precedence: 1)
    /// Flip, 2) Slide, 3) Resize.
    ///
    pub fn constraint_adjustment(&self) -> ConstraintAdjustment::Flags {
        ConstraintAdjustment::Flags::from_bits_truncate(
            unsafe {
                ffi::wlc_view_positioner_get_constraint_adjustment(
                    handle_pos(self)
                )
            }
        )
    }

    /// Get the gravity of the `Positioner`
    ///
    /// Defines in what direction a `View` should be positioned, relative to
    /// the anchor point of the parent `View`.
    /// If two orthogonal gravities are specified (e.g. 'bottom' and 'right'),
    /// then the child surface will be placed in the specified direction;
    /// otherwise, the child surface will be centered over the anchor point on
    /// any axis that had no gravity specified.
    pub fn gravity(&self) -> Gravity::Flags {
        Gravity::Flags::from_bits_truncate(unsafe { ffi::wlc_view_positioner_get_gravity(handle_pos(self)) })
    }

    /// Get the offset of the `Positioner`
    ///
    /// Specify the surface position offset relative to the position of the
    /// anchor on the anchor rectangle and the anchor on the `View`.
    /// For example if the anchor of the anchor rectangle is at (x, y), the
    /// surface has the gravity bottom|right, and the offset is (ox, oy), the
    /// calculated surface position will be (x + ox, y + oy).
    /// The offset position of the surface is the one used for constraint
    /// testing
    pub fn offset(&self) -> Point {
        unsafe { Point::from_ffi(&*ffi::wlc_view_positioner_get_offset(handle_pos(self))) }
    }

    /// Get the size of the `Positioner`
    ///
    /// Expected size of the child `View`
    pub fn size(&self) -> Size {
        unsafe { Size::from_ffi(&*ffi::wlc_view_positioner_get_size(handle_pos(self))) }
    }
}

/// Weak reference to a view
///
/// Can be optained by `view.weak_reference()`
#[derive(Clone)]
pub struct WeakView(Weak<()>, ffi::wlc_handle);
impl WeakView {
    /// Upgrade your weak reference to an actual `View`
    ///
    /// # Safety
    /// This function is unsafe, because it creates a lifetime bound to the
    /// WeakView, which may live forever..
    /// But no view lives forever and might be destroyed by its process at any
    /// time
    /// A disconnection is signaled by `Callback::view_destroyed` and the View
    /// is deallocated shortly after.
    /// Because of this using this function one may create an invalid View
    /// reference.
    /// Also using this function it is possible to optain multiple mutable
    /// references, which is also unsafe
    ///
    /// See `WeakView::run` for a safe variant
    pub unsafe fn upgrade(&self) -> Option<&View> {
        let test = self.0.clone().upgrade();
        match test {
            Some(_) => Some(from_handle(self.1)),
            None => None,
        }
    }

    /// Run a function on the referenced View, if it still exists
    ///
    /// Returns the result of the function, if successful
    ///
    /// # Safety
    /// By enforcing a rather harsh limit on the lifetime of the view
    /// to a short lived scope of an anonymous function,
    /// this function makes sure the view does not live longer then it exists.
    pub fn run<F, R>(&self, runner: F) -> Option<R>
        where F: FnOnce(&View) -> R
    {
        let view = unsafe { self.upgrade() };
        match view {
            Some(view) => Some(runner(view)),
            None => None,
        }
    }
}

impl fmt::Debug for WeakView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WeakView {{ id: {} }}", self.1)
    }
}

#[cfg(not(feature = "unsafe-stable"))]
impl !Send for WeakView {}
#[cfg(not(feature = "unsafe-stable"))]
impl !Sync for WeakView {}

pub fn from_handle<'a>(handle: ffi::wlc_handle) -> &'a mut View {
    unsafe { &mut *(handle as *mut View) }
}

pub fn handle(view: &View) -> ffi::wlc_handle {
    unsafe { mem::transmute(view) }
}

pub fn handle_pos(pos: &Positioner) -> ffi::wlc_handle {
    unsafe { mem::transmute(pos) }
}

impl PartialEq for View {
    fn eq(&self, other: &View) -> bool {
        handle(self) == handle(other)
    }
}
impl Eq for View {}

impl PartialEq<WeakView> for View {
    fn eq(&self, other: &WeakView) -> bool {
        handle(self) == other.1
    }
}

impl PartialEq<View> for WeakView {
    fn eq(&self, other: &View) -> bool {
        self.1 == handle(other)
    }
}

impl PartialEq for WeakView {
    fn eq(&self, other: &WeakView) -> bool {
        self.1 == other.1
    }
}
impl Eq for WeakView {}

use std::hash::*;

impl Hash for View {
    fn hash<H: Hasher>(&self, state: &mut H) {
        handle(self).hash(state);
    }
}

impl Hash for WeakView {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}
