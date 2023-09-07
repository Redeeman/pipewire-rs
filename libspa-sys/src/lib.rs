// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
// FIXME: Remove when https://github.com/rust-lang/rust-bindgen/issues/1651 is closed
#[allow(deref_nullptr)]
/// bindgen-generated definitions
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use bindings::*;

// Manually defined symbols that are manually compiled into a C object file, as they need to be present at link-time.
//
// As SPA is a header-only library, global variables and functions are `static` / `static inline`
// and we need to compile them into a C object ourselves.
//
// Also, wrappers around macros are also present here.
mod type_info;
pub use type_info::*;
mod param;
pub use param::*;
mod param_audio;
pub use param_audio::*;
mod param_video;
pub use param_video::*;
mod debug;
pub use debug::*;
mod pod;
pub use pod::*;
mod utils;
pub use utils::*;
