use std::ffi::{c_int, c_void};

use super::*;

extern "C" {
    #[link_name = "libspa_rs_utils_ringbuffer_init"]
    pub fn spa_ringbuffer_init(rbuf: *mut spa_ringbuffer);

    #[link_name = "libspa_rs_utils_ringbuffer_set_avail"]
    pub fn spa_ringbuffer_set_avail(rbuf: *mut spa_ringbuffer, size: u32);

    #[link_name = "libspa_rs_utils_ringbuffer_get_read_index"]
    pub fn spa_ringbuffer_get_read_index(rbuf: *mut spa_ringbuffer, index: *mut u32) -> c_int;

    #[link_name = "libspa_rs_utils_ringbuffer_read_data"]
    pub fn spa_ringbuffer_read_data(
        rbuf: *mut spa_ringbuffer,
        buffer: *const c_void,
        size: u32,
        offset: u32,
        data: *mut c_void,
        len: u32,
    );

    #[link_name = "libspa_rs_utils_ringbuffer_read_update"]
    pub fn spa_ringbuffer_read_update(rbuf: *mut spa_ringbuffer, index: u32);

    #[link_name = "libspa_rs_utils_ringbuffer_get_write_index"]
    pub fn spa_ringbuffer_get_write_index(rbuf: *mut spa_ringbuffer, index: *mut u32) -> c_int;

    #[link_name = "libspa_rs_utils_ringbuffer_write_data"]
    pub fn spa_ringbuffer_write_data(
        rbuf: *mut spa_ringbuffer,
        buffer: *const c_void,
        size: u32,
        offset: u32,
        data: *mut c_void,
        len: u32,
    );

    #[link_name = "libspa_rs_utils_ringbuffer_write_update"]
    pub fn spa_ringbuffer_write_update(rbuf: *mut spa_ringbuffer, index: u32);
}
