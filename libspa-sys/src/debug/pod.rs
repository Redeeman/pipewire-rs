use std::ffi::{c_int, c_void};

use super::*;

extern "C" {
    #[link_name = "libspa_rs_debugc_pod_value"]
    pub fn spa_debugc_pod_value(
        ctx: *mut spa_debug_context,
        indent: c_int,
        info: *const spa_type_info,
        type_: u32,
        body: *mut c_void,
        size: u32,
    ) -> c_int;

    #[link_name = "libspa_rs_debugc_pod"]
    pub fn spa_debugc_pod(
        ctx: *mut spa_debug_context,
        indent: c_int,
        info: *const spa_type_info,
        pod: *const spa_pod,
    ) -> c_int;

    #[link_name = "libspa_rs_debug_pod_value"]
    pub fn spa_debug_pod_value(
        indent: c_int,
        info: *const spa_type_info,
        type_: u32,
        body: *mut c_void,
        size: u32,
    ) -> c_int;

    #[link_name = "libspa_rs_debug_pod"]
    pub fn spa_debug_pod(indent: c_int, info: *const spa_type_info, pod: *const spa_pod) -> c_int;
}
