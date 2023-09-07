use std::ffi::c_int;

use super::*;

extern "C" {
    #[link_name = "libspa_rs_format_video_raw_parse"]
    pub fn spa_format_video_raw_parse(
        format: *const spa_pod,
        info: *mut spa_video_info_raw,
    ) -> c_int;

    #[link_name = "libspa_rs_format_video_dsp_parse"]
    pub fn spa_format_video_dsp_parse(
        format: *const spa_pod,
        info: *mut spa_video_info_dsp,
    ) -> c_int;

    #[link_name = "libspa_rs_format_video_h264_parse"]
    pub fn spa_format_video_h264_parse(
        format: *const spa_pod,
        info: *mut spa_video_info_h264,
    ) -> c_int;

    #[link_name = "libspa_rs_format_video_mjpg_parse"]
    pub fn spa_format_video_mjpg_parse(
        format: *const spa_pod,
        info: *mut spa_video_info_mjpg,
    ) -> c_int;

    #[link_name = "libspa_rs_format_video_raw_build"]
    pub fn spa_format_video_raw_build(
        builder: *mut spa_pod_builder,
        id: u32,
        info: *mut spa_video_info_raw,
    ) -> *mut spa_pod;

    #[link_name = "libspa_rs_format_video_dsp_build"]
    pub fn spa_format_video_dsp_build(
        builder: *mut spa_pod_builder,
        id: u32,
        info: *mut spa_video_info_dsp,
    ) -> *mut spa_pod;

    #[link_name = "libspa_rs_format_video_h264_build"]
    pub fn spa_format_video_h264_build(
        builder: *mut spa_pod_builder,
        id: u32,
        info: *mut spa_video_info_h264,
    ) -> *mut spa_pod;

    #[link_name = "libspa_rs_format_video_mjpg_build"]
    pub fn spa_format_video_mjpg_build(
        builder: *mut spa_pod_builder,
        id: u32,
        info: *mut spa_video_info_mjpg,
    ) -> *mut spa_pod;
}
