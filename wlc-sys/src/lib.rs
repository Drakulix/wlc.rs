#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;
use libc::c_void;

type wl_resource = c_void;
type wl_client = c_void;
type wl_display = c_void;
type wl_interface = c_void;

include!("gen.rs");
