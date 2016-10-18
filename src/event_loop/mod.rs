//! Exposed functionality of wlc's underlying run loop
//!
//! XXX: IO largely untested. Please report errors

/// Events emitted by the run loop
#[allow(non_snake_case, non_upper_case_globals)]
pub mod Event {
    use ffi;
    #[allow(missing_docs)]
    bitflags! {
        /// Bitmap that may represent multiple Event types
        #[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
        pub flags Flags: u32 {
            /// Did become readable
            const Readable  = ffi::wlc_event_bit_WLC_EVENT_READABLE,
            /// Did become writable
            const Writable  = ffi::wlc_event_bit_WLC_EVENT_WRITABLE,
            /// Did hung up
            const HangUp    = ffi::wlc_event_bit_WLC_EVENT_HANGUP,
            /// An error happened
            const Error     = ffi::wlc_event_bit_WLC_EVENT_ERROR,
        }
    }
}

mod timer;
mod io;

pub use self::io::*;
pub use self::timer::*;
