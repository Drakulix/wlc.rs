# [wlc](https://github.com/Cloudef/wlc) Bindings for Rust [![Build Status](https://travis-ci.org/Drakulix/wlc.svg?branch=master)](https://travis-ci.org/Drakulix/wlc) [![Crates.io](https://img.shields.io/crates/v/wlc.svg)](https://crates.io/crates/simplelog) [![Crates.io](https://img.shields.io/crates/l/wlc.svg)](https://crates.io/crates/simplelog) [![](https://tokei.rs/b1/github/Drakulix/wlc)](https://github.com/Aaronepower/tokei)

Completely safe and idiomatic bindings to the wayland compositor library.

## [Documentation](https://drakulix.github.io/wlc)

## Example

```rust
// for a more functional example see /examples/example.rs
use wlc::*;

struct Compositor;
impl Callback for Compositor
{
    fn view_created(&mut self, view: &View) -> bool
    {
        view.set_visibility(view.output().visibility());
        view.bring_to_front();
        view.focus();
        true
    }

    fn view_focus(&mut self, view: &View, focus: bool)
    {
        view.set_state(ViewState::Activated, focus);
    }
}

fn main()
{
    wlc::init(Compositor).unwrap()
}
```

## Usage

This crate currently requires `nightly` Rust to mark certain ffi-related `struct`s explicitly as not `Send`.

You can opt-out of this behaviour with a feature-flag (unsafe-stable).
Make you never pass `View`, `Output` including their Weak-Variants, `Positioner` or `Wlc` to another thread.


Add to your Cargo.toml
```
wlc = "0.1"
```

For stable
```
wlc = { version = "1.0", features = "unsafe-stable" }
```

For static compilation (combination is possible)
```
wlc = { version = "1.0", features = "static" }
```
See [wlc](https://github.com/Cloudef/wlc) for build dependencies, when doing a static build.


Additionally the features `render` and `wayland` enable the optional extensions wlc provides.

In that case `WlcSurface`, `WlcSubSurface` and `GLES2Renderer` should also not be send across threads, when using `unsafe-stable`.


### A note on [rust-wlc](https://github.com/Immington-Industries/rust-wlc)

rust-wlc has some short comings this crate tries to avoid. It was build without any parts of the original rust-wlc source code and may have its own problems, but it tries to deal with the following issues differently:

(In the following statements `wlc` refers to the original C Library and `rust-wlc` to the alternative wlc bindings)

- wlc does not transfer the ownership of views and output structs to the implementing compositor. Instead any view or output might be deallocated by the library after a `view_destroyed`/`output_destroyed` callback. rust-wlc does not model this relationship correctly in my opinion. See [DESIGN.md](https://github.com/Drakulix/wlc.rs/tree/master/DESIGN.md) to understand how this library models `View` and `Output`.
- rust-wlc lets you use `extern` functions and directly interfere with C-code. This implementation almost requires a global singleton and the usage of `lazy_static`. This crate provides you with a Trait to be used for you custom compositor and hides these implementation details.
- This crate provides a safer alternative to `wlc`'s userdata API. It is still unsafe in some aspects and should be abstracted by any compositor implementation, but it is easier to handle.
- Exposes run loop functions.
- This crate implements most of wlc's render and wayland api's.
- rust-wlc is most likely better tested, as it has likely some more users and a simple mocking library (if I am correct). So please report any issues you may find.

Please note, that I do not try to compete with `rust-wlc` in anyway. I also respect the work they have done and their window manager `way-cooler` is certainly an interesting project.
I just did not like their design decisions and decided to build my own wlc-bindings and my own [window manager](https://github.com/Drakulix/fireplace).
