extern crate bindgen;
#[cfg(feature = "static")]
extern crate cmake;

fn main()
{
    // Setup bindings builder
    let generated = bindgen::builder()
        .header("src/wlc.h")
        .hide_type(r"^wl_.*$")
        .whitelisted_type(r"^wlc_.*$")
        .whitelisted_function(r"^wlc_.*$")
        .no_unstable_rust()
        .ctypes_prefix("libc")
        .constified_enum("wlc_log_type")
        .constified_enum("wlc_backend_type")
        .constified_enum("wlc_event_bit")
        .constified_enum("wlc_view_state_bit")
        .constified_enum("wlc_view_type_bit")
        .constified_enum("wlc_view_property_update_bit")
        .constified_enum("wlc_resize_edge")
        .constified_enum("wlc_modifier_bit")
        .constified_enum("wlc_led_bit")
        .constified_enum("wlc_key_state")
        .constified_enum("wlc_button_state")
        .constified_enum("wlc_scroll_axis_bit")
        .constified_enum("wlc_touch_type")
        .constified_enum("wlc_positioner_anchor_bit")
        .constified_enum("wlc_positioner_gravity_bit")
        .constified_enum("wlc_positioner_constraint_adjustment_bit")
        .constified_enum("wlc_surface_format")
        .constified_enum("wlc_renderer")
        .constified_enum("wlc_pixel_format")
        .clang_arg("-I")
        .clang_arg("wlc/include")
        .generate().unwrap();

    if cfg!(feature = "static") {
        println!("cargo:rustc-link-lib=dylib=dbus-1");
        println!("cargo:rustc-link-lib=dylib=systemd");
        println!("cargo:rustc-link-lib=dylib=wayland-server");
        println!("cargo:rustc-link-lib=dylib=udev");
        println!("cargo:rustc-link-lib=dylib=input");
        println!("cargo:rustc-link-lib=dylib=pixman-1");
        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=EGL");
        println!("cargo:rustc-link-lib=dylib=gbm");
        println!("cargo:rustc-link-lib=dylib=X11");
        println!("cargo:rustc-link-lib=dylib=xcb");
        println!("cargo:rustc-link-lib=dylib=xcb-composite");
        println!("cargo:rustc-link-lib=dylib=xcb-xfixes");
        println!("cargo:rustc-link-lib=dylib=xcb-xkb");
        println!("cargo:rustc-link-lib=dylib=X11-xcb");
        println!("cargo:rustc-link-lib=dylib=xcb-image");
        println!("cargo:rustc-link-lib=dylib=drm");
    } else {
        println!("cargo:rustc-link-lib=dylib=wlc");
    }

    // Generate the bindings
    generated.write_to_file("src/gen.rs").unwrap();

    cmake();
}

#[cfg(not(feature = "static"))]
fn cmake() {}

#[cfg(feature = "static")]
fn cmake() {
    use cmake::Config;

    let dst = Config::new("wlc")
                .define("CMAKE_BUILD_TYPE", "Release")
                .define("SOURCE_WLPROTO", "ON")
                .define("WLC_BUILD_STATIC", "ON")
                .define("WLC_BUILD_EXAMPLES", "OFF")
                .define("WLC_BUILD_TESTS", "OFF")
                .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-search=native={}/build/protos", dst.display());
    println!("cargo:rustc-link-search=native={}/build/lib/chck/lib", dst.display());

    println!("cargo:rustc-link-lib=static=wlc");
    println!("cargo:rustc-link-lib=static=wlc-protos");
    println!("cargo:rustc-link-lib=static=chck-atlas");
    println!("cargo:rustc-link-lib=static=chck-buffer");
    println!("cargo:rustc-link-lib=static=chck-dl");
    println!("cargo:rustc-link-lib=static=chck-fs");
    println!("cargo:rustc-link-lib=static=chck-lut");
    println!("cargo:rustc-link-lib=static=chck-pool");
    println!("cargo:rustc-link-lib=static=chck-sjis");
    println!("cargo:rustc-link-lib=static=chck-string");
    println!("cargo:rustc-link-lib=static=chck-tqueue");
    println!("cargo:rustc-link-lib=static=chck-unicode");
    println!("cargo:rustc-link-lib=static=chck-xdg");
}
