#![allow(non_camel_case_types, non_upper_case_globals, missing_docs)]
use ffi;

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Point {
    pub x: i32,
    pub y: i32,
}


#[cfg_attr(feature = "cargo-clippy", allow(if_same_then_else))]
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        if self.x < other.x && self.y <= other.y {
            Some(Ordering::Less)
        } else if self.y < other.y && self.x <= other.x {
            Some(Ordering::Less)
        } else if self.x > other.x && self.y >= other.y {
            Some(Ordering::Greater)
        } else if self.y > other.y && self.x >= other.x {
            Some(Ordering::Greater)
        } else if self.x == other.x && self.y == other.y {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

#[cfg_attr(feature = "cargo-clippy", allow(if_same_then_else))]
impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Size) -> Option<Ordering> {
        if self.w < other.w && self.h <= other.h {
            Some(Ordering::Less)
        } else if self.h < other.h && self.w <= other.w {
            Some(Ordering::Less)
        } else if self.w > other.w && self.h >= other.h {
            Some(Ordering::Greater)
        } else if self.h > other.h && self.w >= other.w {
            Some(Ordering::Greater)
        } else if self.w == other.w && self.h == other.h {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Geometry {
    pub origin: Point,
    pub size: Size,
}

#[cfg_attr(feature = "cargo-clippy", allow(if_same_then_else))]
impl PartialOrd for Geometry {
    fn partial_cmp(&self, other: &Geometry) -> Option<Ordering> {
        if self.origin < other.origin && self.size <= other.size {
            Some(Ordering::Less)
        } else if self.size < other.size && self.origin <= other.origin {
            Some(Ordering::Less)
        } else if self.origin > other.origin && self.size >= other.size {
            Some(Ordering::Greater)
        } else if self.size > other.size && self.origin >= other.origin {
            Some(Ordering::Greater)
        } else if self.origin == other.origin && self.size == other.size {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl Point {
    #[doc(hidden)]
    pub fn into_ffi(self) -> ffi::wlc_point {
        ffi::wlc_point {
            x: self.x,
            y: self.y,
        }
    }

    #[doc(hidden)]
    pub fn from_ffi(point: &ffi::wlc_point) -> Point {
        Point {
            x: point.x,
            y: point.y,
        }
    }
}

impl Size {
    #[doc(hidden)]
    pub fn into_ffi(self) -> ffi::wlc_size {
        ffi::wlc_size {
            w: self.w,
            h: self.h,
        }
    }

    #[doc(hidden)]
    pub fn from_ffi(size: &ffi::wlc_size) -> Size {
        Size {
            w: size.w,
            h: size.h,
        }
    }
}

impl Geometry {
    #[doc(hidden)]
    pub fn into_ffi(self) -> ffi::wlc_geometry {
        ffi::wlc_geometry {
            origin: self.origin.into_ffi(),
            size: self.size.into_ffi(),
        }
    }

    #[doc(hidden)]
    pub fn from_ffi(geo: &ffi::wlc_geometry) -> Geometry {
        Geometry {
            origin: Point::from_ffi(&geo.origin),
            size: Size::from_ffi(&geo.size),
        }
    }
}

enum_from_primitive! {
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    #[repr(u32)]
    /// Backend used by wlc
    pub enum BackendType
    {
        Null = ffi::wlc_backend_type_WLC_BACKEND_NONE,
        DRM  = ffi::wlc_backend_type_WLC_BACKEND_DRM,
        X11  = ffi::wlc_backend_type_WLC_BACKEND_X11,
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
enum_serde!(BackendType {
    Null,
    DRM,
    X11,
});

/// States a View may hold
#[allow(non_snake_case)]
pub mod ViewState {
    use ffi;
    bitflags! {
        /// Bitmap that may represent multiple states
        pub flags Flags: u32 {
            /// view is maximized
            const Maximized     = ffi::wlc_view_state_bit_WLC_BIT_MAXIMIZED,
            /// view is shown in fullscreen
            const Fullscreen    = ffi::wlc_view_state_bit_WLC_BIT_FULLSCREEN,
            /// view is resizing
            const Resizing      = ffi::wlc_view_state_bit_WLC_BIT_RESIZING,
            /// view is moving
            const Moving        = ffi::wlc_view_state_bit_WLC_BIT_MOVING,
            /// view is active
            const Activated     = ffi::wlc_view_state_bit_WLC_BIT_ACTIVATED,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
bitflags_serde!(ViewState {
    Maximized,
    Fullscreen,
    Resizing,
    Moving,
    Activated,
});

/// Typical view categories
#[allow(non_snake_case)]
pub mod ViewType {
    use ffi;
    bitflags! {
        /// Bitmap that may represent multiple types
        pub flags Flags: u32 {
            /// Override redirect (x11)
            const OverrideRedirect  = ffi::wlc_view_type_bit_WLC_BIT_OVERRIDE_REDIRECT,
            /// Tooltips, DnD's, menus (x11)
            const Unmanaged         = ffi::wlc_view_type_bit_WLC_BIT_UNMANAGED,
            /// Splash screens (x11)
            const Splash            = ffi::wlc_view_type_bit_WLC_BIT_SPLASH,
            /// Modal windows (x11)
            const Modal             = ffi::wlc_view_type_bit_WLC_BIT_MODAL,
            /// xdg-shell, wl-shell popups
            const Popup             = ffi::wlc_view_type_bit_WLC_BIT_POPUP,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
bitflags_serde!(ViewType {
    OverrideRedirect,
    Unmanaged,
    Splash,
    Modal,
    Popup,
});

/// Updated properties of a view
#[allow(non_snake_case)]
pub mod ViewPropertyUpdate {
    use ffi;
    bitflags! {
        /// Bitmap that may represent multiple updated properties
        pub flags Flags: u32 {
            /// Title changed. Receive with `view.title()`
            const Title     = ffi::wlc_view_property_update_bit_WLC_BIT_PROPERTY_TITLE,
            /// Class changed. Receive with `view.class()`
            const Class     = ffi::wlc_view_property_update_bit_WLC_BIT_PROPERTY_CLASS,
            /// AppID changed. Receive with `view.app_id()`
            const AppID     = ffi::wlc_view_property_update_bit_WLC_BIT_PROPERTY_APP_ID,
            /// PID of belonging process changed. Receive with `view.pid()`
            const PID       = ffi::wlc_view_property_update_bit_WLC_BIT_PROPERTY_PID,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
bitflags_serde!(ViewPropertyUpdate {
    Title,
    Class,
    AppID,
    PID,
});

/// Edges for interactive resizing
#[allow(non_snake_case)]
pub mod ResizeEdge {
    use ffi;
    bitflags! {
        /// Bitmap that may represent multiple edges
        pub flags Flags: u32 {
            const Null         = ffi::wlc_resize_edge_WLC_RESIZE_EDGE_NONE,
            const Top          = ffi::wlc_resize_edge_WLC_RESIZE_EDGE_TOP,
            const Bottom       = ffi::wlc_resize_edge_WLC_RESIZE_EDGE_BOTTOM,
            const Left         = ffi::wlc_resize_edge_WLC_RESIZE_EDGE_LEFT,
            const Right        = ffi::wlc_resize_edge_WLC_RESIZE_EDGE_RIGHT,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
bitflags_serde!(ResizeEdge {
    Null,
    Top,
    Bottom,
    Left,
    Right,
});

/// Set of modifiers on a keyboard
#[allow(non_snake_case)]
pub mod Modifier {
    use ffi;
    bitflags! {
        /// Bitmap that may represent multiple modifiers
        pub flags Flags: u32 {
            /// Shift Key
            const Shift     = ffi::wlc_modifier_bit_WLC_BIT_MOD_SHIFT,
            /// Caps Key
            const Caps      = ffi::wlc_modifier_bit_WLC_BIT_MOD_CAPS,
            /// Control Key
            const Ctrl      = ffi::wlc_modifier_bit_WLC_BIT_MOD_CTRL,
            /// Alternative Function Key
            const Alt       = ffi::wlc_modifier_bit_WLC_BIT_MOD_ALT,
            /// Second Modifier
            const Mod2      = ffi::wlc_modifier_bit_WLC_BIT_MOD_MOD2,
            /// Third Modifier
            const Mod3      = ffi::wlc_modifier_bit_WLC_BIT_MOD_MOD3,
            /// Logo (Forth) Modifier (Windows/Command Key)
            const Logo      = ffi::wlc_modifier_bit_WLC_BIT_MOD_LOGO,
            /// Fifth Modifier
            const Mod5      = ffi::wlc_modifier_bit_WLC_BIT_MOD_MOD5,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
bitflags_serde!(Modifier {
    Shift,
    Caps,
    Ctrl,
    Alt,
    Mod2,
    Mod3,
    Logo,
    Mod5,
});

/// Keyboard LEDs
#[allow(non_snake_case)]
pub mod Led {
    use ffi;
    bitflags! {
        /// Bitmap that may represent multiple leds
        pub flags Flags: u32 {
            const Num       = ffi::wlc_led_bit_WLC_BIT_LED_NUM,
            const Caps      = ffi::wlc_led_bit_WLC_BIT_LED_CAPS,
            const Scroll    = ffi::wlc_led_bit_WLC_BIT_LED_SCROLL,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
bitflags_serde!(Led {
    Num,
    Caps,
    Scroll,
});

enum_from_primitive! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(u32)]
    /// States of a key
    pub enum KeyState {
        Released = ffi::wlc_key_state_WLC_KEY_STATE_RELEASED,
        Pressed  = ffi::wlc_key_state_WLC_KEY_STATE_PRESSED,
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
enum_serde!(KeyState {
    Released,
    Pressed,
});

enum_from_primitive! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(u32)]
    /// States of a button
    pub enum ButtonState {
        Released = ffi::wlc_button_state_WLC_BUTTON_STATE_RELEASED,
        Pressed  = ffi::wlc_button_state_WLC_BUTTON_STATE_PRESSED,
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
enum_serde!(ButtonState {
    Released,
    Pressed,
});

/// Scroll Axes
#[allow(non_snake_case)]
pub mod ScrollAxis {
    use ffi;
    bitflags! {
        /// Bitmap that may represent multiple axis
        pub flags Flags: u8 {
            /// Vertical Scroll Axis
            const Vertical      = ffi::wlc_scroll_axis_bit_WLC_SCROLL_AXIS_VERTICAL as u8,
            /// Horizontal Scroll Axis
            const Horizontal    = ffi::wlc_scroll_axis_bit_WLC_SCROLL_AXIS_HORIZONTAL as u8,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
bitflags_serde!(ScrollAxis {
    Vertical,
    Horizontal,
});

enum_from_primitive! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(u32)]
    /// Touch events
    pub enum TouchType {
        Down    = ffi::wlc_touch_type_WLC_TOUCH_DOWN,
        Up      = ffi::wlc_touch_type_WLC_TOUCH_UP,
        Motion  = ffi::wlc_touch_type_WLC_TOUCH_MOTION,
        Frame   = ffi::wlc_touch_type_WLC_TOUCH_FRAME,
        Cancel  = ffi::wlc_touch_type_WLC_TOUCH_CANCEL,
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
enum_serde!(TouchType {
    Down,
    Up,
    Motion,
    Frame,
    Cancel,
});

/// Combined keyboard Modifiers and LEDs
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Modifiers {
    pub leds: Led::Flags,
    pub mods: Modifier::Flags,
}

impl Modifiers {
    #[doc(hidden)]
    pub fn into_ffi(self) -> ffi::wlc_modifiers {
        ffi::wlc_modifiers {
            leds: self.leds.bits(),
            mods: self.mods.bits(),
        }
    }

    #[doc(hidden)]
    pub fn from_ffi(mods: &ffi::wlc_modifiers) -> Modifiers {
        Modifiers {
            leds: Led::Flags::from_bits_truncate(mods.leds),
            mods: Modifier::Flags::from_bits_truncate(mods.mods),
        }
    }

    pub fn empty() -> Modifiers {
        Modifiers {
            leds: Led::Flags::empty(),
            mods: Modifier::Flags::empty(),
        }
    }
}

/// Visibility Flags
/// Useful to implement workspace functionality or simply hiding views
#[allow(non_snake_case)]
pub mod Visibility {
    bitflags! {
        /// Bitmap that may represent multiple Slots for Visibility
        pub flags Flags: u32 {
            const Null   = 0b00000000000000000000000000000000,
            const Slot1  = 0b00000000000000000000000000000001,
            const Slot2  = 0b00000000000000000000000000000010,
            const Slot3  = 0b00000000000000000000000000000100,
            const Slot4  = 0b00000000000000000000000000001000,
            const Slot5  = 0b00000000000000000000000000010000,
            const Slot6  = 0b00000000000000000000000000100000,
            const Slot7  = 0b00000000000000000000000001000000,
            const Slot8  = 0b00000000000000000000000010000000,
            const Slot9  = 0b00000000000000000000000100000000,
            const Slot10 = 0b00000000000000000000001000000000,
            const Slot11 = 0b00000000000000000000010000000000,
            const Slot12 = 0b00000000000000000000100000000000,
            const Slot13 = 0b00000000000000000001000000000000,
            const Slot14 = 0b00000000000000000010000000000000,
            const Slot15 = 0b00000000000000000100000000000000,
            const Slot16 = 0b00000000000000001000000000000000,
            const Slot17 = 0b00000000000000010000000000000000,
            const Slot18 = 0b00000000000000100000000000000000,
            const Slot19 = 0b00000000000001000000000000000000,
            const Slot20 = 0b00000000000010000000000000000000,
            const Slot21 = 0b00000000000100000000000000000000,
            const Slot22 = 0b00000000001000000000000000000000,
            const Slot23 = 0b00000000010000000000000000000000,
            const Slot24 = 0b00000000100000000000000000000000,
            const Slot25 = 0b00000001000000000000000000000000,
            const Slot26 = 0b00000010000000000000000000000000,
            const Slot27 = 0b00000100000000000000000000000000,
            const Slot28 = 0b00001000000000000000000000000000,
            const Slot29 = 0b00010000000000000000000000000000,
            const Slot30 = 0b00100000000000000000000000000000,
            const Slot31 = 0b01000000000000000000000000000000,
            const Slot32 = 0b10000000000000000000000000000000,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
bitflags_serde!(Visibility {
    Null,
    Slot1,
    Slot2,
    Slot3,
    Slot4,
    Slot5,
    Slot6,
    Slot7,
    Slot8,
    Slot9,
    Slot10,
    Slot11,
    Slot12,
    Slot13,
    Slot14,
    Slot15,
    Slot16,
    Slot17,
    Slot18,
    Slot19,
    Slot20,
    Slot21,
    Slot22,
    Slot23,
    Slot24,
    Slot25,
    Slot26,
    Slot27,
    Slot28,
    Slot29,
    Slot30,
    Slot31,
    Slot32,
});

#[allow(non_snake_case)]
/// View Positioner Anchor
pub mod Anchor {
    use ffi;
    bitflags! {
        /// Bitmap that may represent multiple Anchors of a `View` by it's `Positioner`
        pub flags Flags: u32 {
            const Null   = ffi::wlc_positioner_anchor_bit_WLC_BIT_ANCHOR_NONE,
            const Top    = ffi::wlc_positioner_anchor_bit_WLC_BIT_ANCHOR_TOP,
            const Bottom = ffi::wlc_positioner_anchor_bit_WLC_BIT_ANCHOR_BOTTOM,
            const Left   = ffi::wlc_positioner_anchor_bit_WLC_BIT_ANCHOR_LEFT,
            const Right  = ffi::wlc_positioner_anchor_bit_WLC_BIT_ANCHOR_RIGHT,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
bitflags_serde!(Anchor {
    Null,
    Top,
    Bottom,
    Left,
    Right,
});

#[allow(non_snake_case)]
/// View Positioner Gravity
pub mod Gravity {
    use ffi;
    bitflags! {
        /// Bitmap that may represent multiple Gravity Values for a `View` by it's `Positioner`
        pub flags Flags: u32 {
            const Null   = ffi::wlc_positioner_gravity_bit_WLC_BIT_GRAVITY_NONE,
            const Top    = ffi::wlc_positioner_gravity_bit_WLC_BIT_GRAVITY_TOP,
            const Bottom = ffi::wlc_positioner_gravity_bit_WLC_BIT_GRAVITY_BOTTOM,
            const Left   = ffi::wlc_positioner_gravity_bit_WLC_BIT_GRAVITY_LEFT,
            const Right  = ffi::wlc_positioner_gravity_bit_WLC_BIT_GRAVITY_RIGHT,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
bitflags_serde!(Gravity {
    Null,
    Top,
    Bottom,
    Left,
    Right,
});

#[allow(non_snake_case)]
/// View Positioner Adjustments when View is constraint
pub mod ConstraintAdjustment {
    use ffi;
    bitflags! {
        /// Bitmap that may represent multiple Constraints to adjust a `View` by it's `Positioner`
        pub flags Flags: u32 {
            /// Don't alter the surface position even if it is
            /// constrained on some axis, for example partially outside
            /// the edge of a monitor.
            const Null =
                ffi::wlc_positioner_constraint_adjustment_bit_WLC_BIT_CONSTRAINT_ADJUSTMENT_NONE,
            /// Invert the anchor and gravity on the x axis if the
            /// surface is constrained on the x axis. For example, if the
            /// left edge of the surface is constrained, the gravity is
            /// 'left' and the anchor is 'left', change the gravity to
            /// 'right' and the anchor to 'right'.
            ///
	        /// If the adjusted position also ends up being constrained,
            /// the resulting position of the flip_x adjustment will be
            /// the one before the adjustment.
            const FlipX =
                ffi::wlc_positioner_constraint_adjustment_bit_WLC_BIT_CONSTRAINT_ADJUSTMENT_FLIP_X,
            /// Invert the anchor and gravity on the y axis if the
            /// surface is constrained on the y axis. For example, if the
            /// bottom edge of the surface is constrained, the gravity is
            /// 'bottom' and the anchor is 'bottom', change the gravity
            /// to 'top' and the anchor to 'top'.
            ///
	        /// If the adjusted position also ends up being constrained,
            /// the resulting position of the flip_y adjustment will be
            /// the one before the adjustment.
            const FlipY =
                ffi::wlc_positioner_constraint_adjustment_bit_WLC_BIT_CONSTRAINT_ADJUSTMENT_FLIP_Y,
            /// Resize the surface horizontally so that it is completely
            /// unconstrained.
            const ResizeX =
                ffi::wlc_positioner_constraint_adjustment_bit_WLC_BIT_CONSTRAINT_ADJUSTMENT_RESIZE_X,
            /// Resize the surface vertically so that it is completely
            /// unconstrained.
            const ResizeY =
                ffi::wlc_positioner_constraint_adjustment_bit_WLC_BIT_CONSTRAINT_ADJUSTMENT_RESIZE_Y,
            /// Slide the surface along the x axis until it is no longer
            /// constrained.
            ///
	        /// First try to slide towards the direction of the gravity
            /// on the x axis until either the edge in the opposite
            /// direction of the gravity is unconstrained or the edge in
            /// the direction of the gravity is constrained.
            ///
	        /// Then try to slide towards the opposite direction of the
            /// gravity on the x axis until either the edge in the
            /// direction of the gravity is unconstrained or the edge in
            /// the opposite direction of the gravity is constrained.
            const SlideX =
                ffi::wlc_positioner_constraint_adjustment_bit_WLC_BIT_CONSTRAINT_ADJUSTMENT_SLIDE_X,
            /// Slide the surface along the y axis until it is no longer
            /// constrained.
            ///
	        /// First try to slide towards the direction of the gravity
            /// on the y axis until either the edge in the opposite
            /// direction of the gravity is unconstrained or the edge in
            /// the direction of the gravity is constrained.
            ///
	        /// Then try to slide towards the opposite direction of the
            /// gravity on the y axis until either the edge in the
            /// direction of the gravity is unconstrained or the edge in
            /// the opposite direction of the gravity is constrained.
            const SlideY =
                ffi::wlc_positioner_constraint_adjustment_bit_WLC_BIT_CONSTRAINT_ADJUSTMENT_SLIDE_Y,
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
bitflags_serde!(ConstraintAdjustment {
    Null,
    FlipX,
    FlipY,
    ResizeX,
    ResizeY,
    SlideX,
    SlideY,
});

#[test]
fn test_ord_point() {
    {
        let p1 = Point { x: 100, y: 200 };
        let p2 = Point { x: 100, y: 300 };
        assert!(p1 < p2);
    }

    {
        let p1 = Point { x: 100, y: 200 };
        let p2 = Point { x: 200, y: 300 };
        assert!(p1 < p2);
    }

    {
        let p1 = Point { x: 100, y: 200 };
        let p2 = Point { x: 200, y: 200 };
        assert!(p1 < p2);
    }

    {
        let p1 = Point { x: 200, y: 300 };
        let p2 = Point { x: 100, y: 300 };
        assert!(p1 > p2);
    }

    {
        let p1 = Point { x: 200, y: 400 };
        let p2 = Point { x: 100, y: 300 };
        assert!(p1 > p2);
    }

    {
        let p1 = Point { x: 200, y: 300 };
        let p2 = Point { x: 200, y: 200 };
        assert!(p1 > p2);
    }

    {
        let p1 = Point { x: 100, y: 300 };
        let p2 = Point { x: 300, y: 100 };
        assert_eq!(p1.partial_cmp(&p2), None);
    }

    {
        let p1 = Point { x: 300, y: 100 };
        let p2 = Point { x: 100, y: 300 };
        assert_eq!(p1.partial_cmp(&p2), None);
    }
}

#[test]
fn test_ord_size() {
    {
        let s1 = Size { w: 100, h: 200 };
        let s2 = Size { w: 100, h: 300 };
        assert!(s1 < s2);
    }

    {
        let s1 = Size { w: 100, h: 200 };
        let s2 = Size { w: 200, h: 300 };
        assert!(s1 < s2);
    }

    {
        let s1 = Size { w: 100, h: 200 };
        let s2 = Size { w: 200, h: 200 };
        assert!(s1 < s2);
    }

    {
        let s1 = Size { w: 200, h: 300 };
        let s2 = Size { w: 100, h: 300 };
        assert!(s1 > s2);
    }

    {
        let s1 = Size { w: 200, h: 400 };
        let s2 = Size { w: 100, h: 300 };
        assert!(s1 > s2);
    }

    {
        let s1 = Size { w: 200, h: 300 };
        let s2 = Size { w: 200, h: 200 };
        assert!(s1 > s2);
    }

    {
        let s1 = Size { w: 100, h: 300 };
        let s2 = Size { w: 300, h: 100 };
        assert_eq!(s1.partial_cmp(&s2), None);
    }

    {
        let s1 = Size { w: 300, h: 100 };
        let s2 = Size { w: 100, h: 300 };
        assert_eq!(s1.partial_cmp(&s2), None);
    }
}

#[test]
fn test_ord_geo() {
    {
        let g1 = Geometry {
            origin: Point { x: 0, y: 200 },
            size: Size { w: 100, h: 200 },
        };
        let g2 = Geometry {
            origin: Point { x: 100, y: 200 },
            size: Size { w: 100, h: 200 },
        };
        assert!(g1 < g2);
    }

    {
        let g1 = Geometry {
            origin: Point { x: 0, y: 200 },
            size: Size { w: 100, h: 200 },
        };
        let g2 = Geometry {
            origin: Point { x: 0, y: 200 },
            size: Size { w: 100, h: 300 },
        };
        assert!(g1 < g2);
    }

    {
        let g1 = Geometry {
            origin: Point { x: 100, y: 200 },
            size: Size { w: 100, h: 200 },
        };
        let g2 = Geometry {
            origin: Point { x: 0, y: 200 },
            size: Size { w: 100, h: 200 },
        };
        assert!(g1 > g2);
    }

    {
        let g1 = Geometry {
            origin: Point { x: 100, y: 200 },
            size: Size { w: 100, h: 300 },
        };
        let g2 = Geometry {
            origin: Point { x: 100, y: 200 },
            size: Size { w: 100, h: 200 },
        };
        assert!(g1 > g2);
    }

    {
        let g1 = Geometry {
            origin: Point { x: 200, y: 200 },
            size: Size { w: 100, h: 300 },
        };
        let g2 = Geometry {
            origin: Point { x: 100, y: 200 },
            size: Size { w: 100, h: 400 },
        };
        assert_eq!(g1.partial_cmp(&g2), None);
    }
}
