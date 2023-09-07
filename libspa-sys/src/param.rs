use std::ffi::c_int;

use super::*;

extern "C" {
    #[link_name = "libspa_rs_format_parse"]
    pub fn spa_format_parse(
        format: *const spa_pod,
        media_type: *mut u32,
        media_subtype: *mut u32,
    ) -> c_int;
}
