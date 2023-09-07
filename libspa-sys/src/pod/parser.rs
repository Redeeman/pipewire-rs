use std::ffi::{c_char, c_double, c_float, c_int, c_void};

use super::*;

extern "C" {
    #[link_name = "libspa_rs_pod_parser_init"]
    pub fn spa_pod_parser_init(parser: *mut spa_pod_parser, data: *const c_void, size: u32);

    #[link_name = "libspa_rs_pod_parser_pod"]
    pub fn spa_pod_parser_pod(parser: *mut spa_pod_parser, pod: *const spa_pod);

    #[link_name = "libspa_rs_pod_parser_get_state"]
    pub fn spa_pod_parser_get_state(parser: *mut spa_pod_parser, state: *mut spa_pod_parser_state);

    #[link_name = "libspa_rs_pod_parser_reset"]
    pub fn spa_pod_parser_reset(parser: *mut spa_pod_parser, state: *mut spa_pod_parser_state);

    #[link_name = "libspa_rs_pod_parser_deref"]
    pub fn spa_pod_parser_deref(
        parser: *mut spa_pod_parser,
        offset: u32,
        size: u32,
    ) -> *mut spa_pod;

    #[link_name = "libspa_rs_pod_parser_frame"]
    pub fn spa_pod_parser_frame(
        parser: *mut spa_pod_parser,
        frame: *mut spa_pod_frame,
    ) -> *mut spa_pod;

    #[link_name = "libspa_rs_pod_parser_push"]
    pub fn spa_pod_parser_push(
        parser: *mut spa_pod_parser,
        frame: *mut spa_pod_frame,
        pod: *const spa_pod,
        offset: u32,
    );

    #[link_name = "libspa_rs_pod_parser_current"]
    pub fn spa_pod_parser_current(parser: *mut spa_pod_parser) -> *mut spa_pod;

    #[link_name = "libspa_rs_pod_parser_advance"]
    pub fn spa_pod_parser_advance(parser: *mut spa_pod_parser, pod: *const spa_pod);

    #[link_name = "libspa_rs_pod_parser_next"]
    pub fn spa_pod_parser_next(parser: *mut spa_pod_parser) -> *mut spa_pod;

    #[link_name = "libspa_rs_pod_parser_pop"]
    pub fn spa_pod_parser_pop(parser: *mut spa_pod_parser, frame: *mut spa_pod_frame) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_bool"]
    pub fn spa_pod_parser_get_bool(parser: *mut spa_pod_parser, value: *mut bool) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_id"]
    pub fn spa_pod_parser_get_id(parser: *mut spa_pod_parser, value: *mut u32) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_int"]
    pub fn spa_pod_parser_get_int(parser: *mut spa_pod_parser, value: *mut i32) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_long"]
    pub fn spa_pod_parser_get_long(parser: *mut spa_pod_parser, value: *mut i64) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_float"]
    pub fn spa_pod_parser_get_float(parser: *mut spa_pod_parser, value: *mut c_float) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_double"]
    pub fn spa_pod_parser_get_double(parser: *mut spa_pod_parser, value: *mut c_double) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_string"]
    pub fn spa_pod_parser_get_string(
        parser: *mut spa_pod_parser,
        value: *mut *const c_char,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_bytes"]
    pub fn spa_pod_parser_get_bytes(
        parser: *mut spa_pod_parser,
        value: *mut *const c_void,
        len: *mut u32,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_pointer"]
    pub fn spa_pod_parser_get_pointer(
        parser: *mut spa_pod_parser,
        _type: *mut u32,
        value: *mut *const c_void,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_fd"]
    pub fn spa_pod_parser_get_fd(parser: *mut spa_pod_parser, value: *mut i64) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_rectangle"]
    pub fn spa_pod_parser_get_rectangle(
        parser: *mut spa_pod_parser,
        value: *mut spa_rectangle,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_fraction"]
    pub fn spa_pod_parser_get_fraction(
        parser: *mut spa_pod_parser,
        value: *mut spa_fraction,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_parser_get_pod"]
    pub fn spa_pod_parser_get_pod(parser: *mut spa_pod_parser, value: *mut *mut spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_parser_push_struct"]
    pub fn spa_pod_parser_push_struct(
        parser: *mut spa_pod_parser,
        frame: *mut spa_pod_frame,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_parser_push_object"]
    pub fn spa_pod_parser_push_object(
        parser: *mut spa_pod_parser,
        frame: *mut spa_pod_frame,
        _type: u32,
        id: *mut u32,
    ) -> c_int;
}
