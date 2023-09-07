use std::ffi::{c_char, c_double, c_float, c_int, c_void};

use super::*;

extern "C" {
    #[link_name = "libspa_rs_pod_is_none"]
    pub fn spa_pod_is_none(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_is_bool"]
    pub fn spa_pod_is_bool(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_get_bool"]
    pub fn spa_pod_get_bool(pod: *const spa_pod, value: *mut bool) -> c_int;

    #[link_name = "libspa_rs_pod_is_id"]
    pub fn spa_pod_is_id(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_get_id"]
    pub fn spa_pod_get_id(pod: *const spa_pod, value: *mut u32) -> c_int;

    #[link_name = "libspa_rs_pod_is_int"]
    pub fn spa_pod_is_int(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_get_int"]
    pub fn spa_pod_get_int(pod: *const spa_pod, value: *mut i32) -> c_int;

    #[link_name = "libspa_rs_pod_is_long"]
    pub fn spa_pod_is_long(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_get_long"]
    pub fn spa_pod_get_long(pod: *const spa_pod, value: *mut i64) -> c_int;

    #[link_name = "libspa_rs_pod_is_float"]
    pub fn spa_pod_is_float(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_get_float"]
    pub fn spa_pod_get_float(pod: *const spa_pod, value: *mut c_float) -> c_int;

    #[link_name = "libspa_rs_pod_is_double"]
    pub fn spa_pod_is_double(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_get_double"]
    pub fn spa_pod_get_double(pod: *const spa_pod, value: *mut c_double) -> c_int;

    #[link_name = "libspa_rs_pod_is_string"]
    pub fn spa_pod_is_string(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_get_string"]
    pub fn spa_pod_get_string(pod: *const spa_pod, value: *mut *const c_char) -> c_int;

    #[link_name = "libspa_rs_pod_is_bytes"]
    pub fn spa_pod_is_bytes(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_get_bytes"]
    pub fn spa_pod_get_bytes(
        pod: *const spa_pod,
        value: *mut *const c_void,
        len: *mut u32,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_is_pointer"]
    pub fn spa_pod_is_pointer(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_get_pointer"]
    pub fn spa_pod_get_pointer(
        pod: *const spa_pod,
        _type: *mut u32,
        value: *mut *const c_void,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_is_fd"]
    pub fn spa_pod_is_fd(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_get_fd"]
    pub fn spa_pod_get_fd(pod: *const spa_pod, value: *mut i64) -> c_int;

    #[link_name = "libspa_rs_pod_is_rectangle"]
    pub fn spa_pod_is_rectangle(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_get_rectangle"]
    pub fn spa_pod_get_rectangle(pod: *const spa_pod, value: *mut spa_rectangle) -> c_int;

    #[link_name = "libspa_rs_pod_is_fraction"]
    pub fn spa_pod_is_fraction(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_get_fraction"]
    pub fn spa_pod_get_fraction(pod: *const spa_pod, value: *mut spa_fraction) -> c_int;

    #[link_name = "libspa_rs_pod_is_bitmap"]
    pub fn spa_pod_is_bitmap(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_is_array"]
    pub fn spa_pod_is_array(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_is_choice"]
    pub fn spa_pod_is_choice(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_is_struct"]
    pub fn spa_pod_is_struct(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_is_object"]
    pub fn spa_pod_is_object(pod: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_is_sequence"]
    pub fn spa_pod_is_sequence(pod: *const spa_pod) -> c_int;
}
