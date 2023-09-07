use std::ffi::{c_char, c_double, c_float, c_int, c_void};

use super::*;

extern "C" {
    #[link_name = "libspa_rs_pod_builder_get_state"]
    pub fn spa_pod_builder_get_state(
        builder: *mut spa_pod_builder,
        state: *mut spa_pod_builder_state,
    );

    #[link_name = "libspa_rs_pod_builder_set_callbacks"]
    pub fn spa_pod_builder_set_callbacks(
        builder: *mut spa_pod_builder,
        callbacks: *const spa_pod_builder_callbacks,
        data: *mut c_void,
    );

    #[link_name = "libspa_rs_pod_builder_reset"]
    pub fn spa_pod_builder_reset(builder: *mut spa_pod_builder, state: *mut spa_pod_builder_state);

    #[link_name = "libspa_rs_pod_builder_init"]
    pub fn spa_pod_builder_init(builder: *mut spa_pod_builder, data: *mut c_void, size: u32);

    #[link_name = "libspa_rs_pod_builder_deref"]
    pub fn spa_pod_builder_deref(builder: *mut spa_pod_builder, offset: u32) -> *mut spa_pod;

    #[link_name = "libspa_rs_pod_builder_frame"]
    pub fn spa_pod_builder_frame(
        builder: *mut spa_pod_builder,
        frame: *mut spa_pod_frame,
    ) -> *mut spa_pod;

    #[link_name = "libspa_rs_pod_builder_push"]
    pub fn spa_pod_builder_push(
        builder: *mut spa_pod_builder,
        frame: *mut spa_pod_frame,
        pod: *const spa_pod,
        offset: u32,
    );

    #[link_name = "libspa_rs_pod_builder_raw"]
    pub fn spa_pod_builder_raw(
        builder: *mut spa_pod_builder,
        data: *const c_void,
        size: u32,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_pad"]
    pub fn spa_pod_builder_pad(builder: *mut spa_pod_builder, size: u32) -> c_int;

    #[link_name = "libspa_rs_pod_builder_raw_padded"]
    pub fn spa_pod_builder_raw_padded(
        builder: *mut spa_pod_builder,
        data: *const c_void,
        size: u32,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_pop"]
    pub fn spa_pod_builder_pop(builder: *mut spa_pod_builder, frame: *mut spa_pod_frame) -> c_void;

    #[link_name = "libspa_rs_pod_builder_primitive"]
    pub fn spa_pod_builder_primitive(builder: *mut spa_pod_builder, p: *const spa_pod) -> c_int;

    #[link_name = "libspa_rs_pod_builder_none"]
    pub fn spa_pod_builder_none(builder: *mut spa_pod_builder) -> c_int;

    #[link_name = "libspa_rs_pod_builder_child"]
    pub fn spa_pod_builder_child(builder: *mut spa_pod_builder, size: u32, _type: u32) -> c_int;

    #[link_name = "libspa_rs_pod_builder_bool"]
    pub fn spa_pod_builder_bool(builder: *mut spa_pod_builder, val: bool) -> c_int;

    #[link_name = "libspa_rs_pod_builder_id"]
    pub fn spa_pod_builder_id(builder: *mut spa_pod_builder, val: u32) -> c_int;

    #[link_name = "libspa_rs_pod_builder_int"]
    pub fn spa_pod_builder_int(builder: *mut spa_pod_builder, val: i32) -> c_int;

    #[link_name = "libspa_rs_pod_builder_long"]
    pub fn spa_pod_builder_long(builder: *mut spa_pod_builder, val: i64) -> c_int;

    #[link_name = "libspa_rs_pod_builder_float"]
    pub fn spa_pod_builder_float(builder: *mut spa_pod_builder, val: c_float) -> c_int;

    #[link_name = "libspa_rs_pod_builder_double"]
    pub fn spa_pod_builder_double(builder: *mut spa_pod_builder, val: c_double) -> c_int;

    #[link_name = "libspa_rs_pod_builder_write_string"]
    pub fn spa_pod_builder_write_string(
        builder: *mut spa_pod_builder,
        str: *const c_char,
        len: u32,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_string_len"]
    pub fn spa_pod_builder_string_len(
        builder: *mut spa_pod_builder,
        str: *const c_char,
        len: u32,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_string"]
    pub fn spa_pod_builder_string(builder: *mut spa_pod_builder, str: *const c_char) -> c_int;

    #[link_name = "libspa_rs_pod_builder_bytes"]
    pub fn spa_pod_builder_bytes(
        builder: *mut spa_pod_builder,
        bytes: *const c_void,
        len: u32,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_reserve_bytes"]
    pub fn spa_pod_builder_reserve_bytes(builder: *mut spa_pod_builder, len: u32) -> *mut c_void;

    #[link_name = "libspa_rs_pod_builder_pointer"]
    pub fn spa_pod_builder_pointer(
        builder: *mut spa_pod_builder,
        _type: u32,
        val: *const c_void,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_fd"]
    pub fn spa_pod_builder_fd(builder: *mut spa_pod_builder, fd: i64) -> c_int;

    #[link_name = "libspa_rs_pod_builder_rectangle"]
    pub fn spa_pod_builder_rectangle(
        builder: *mut spa_pod_builder,
        width: u32,
        height: u32,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_fraction"]
    pub fn spa_pod_builder_fraction(builder: *mut spa_pod_builder, num: u32, denom: u32) -> c_int;

    #[link_name = "libspa_rs_pod_builder_push_array"]
    pub fn spa_pod_builder_push_array(
        builder: *mut spa_pod_builder,
        frame: *mut spa_pod_frame,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_array"]
    pub fn spa_pod_builder_array(
        builder: *mut spa_pod_builder,
        child_size: u32,
        child_type: u32,
        n_elems: u32,
        elems: *const c_void,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_push_choice"]
    pub fn spa_pod_builder_push_choice(
        builder: *mut spa_pod_builder,
        frame: *mut spa_pod_frame,
        _type: u32,
        flags: u32,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_push_struct"]
    pub fn spa_pod_builder_push_struct(
        builder: *mut spa_pod_builder,
        frame: *mut spa_pod_frame,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_push_object"]
    pub fn spa_pod_builder_push_object(
        builder: *mut spa_pod_builder,
        frame: *mut spa_pod_frame,
        _type: u32,
        id: u32,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_prop"]
    pub fn spa_pod_builder_prop(builder: *mut spa_pod_builder, key: u32, flags: u32) -> c_int;

    #[link_name = "libspa_rs_pod_builder_push_sequence"]
    pub fn spa_pod_builder_push_sequence(
        builder: *mut spa_pod_builder,
        frame: *mut spa_pod_frame,
        unit: u32,
    ) -> c_int;

    #[link_name = "libspa_rs_pod_builder_control"]
    pub fn spa_pod_builder_control(builder: *mut spa_pod_builder, offset: u32, _type: u32) -> u32;

    #[link_name = "libspa_rs_choice_from_id"]
    pub fn spa_choice_from_id(id: c_char) -> u32;

    #[link_name = "libspa_rs_pod_copy"]
    pub fn spa_pod_copy(pod: *const spa_pod) -> *mut spa_pod;
}
