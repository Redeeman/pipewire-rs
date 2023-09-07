use std::ffi::c_int;

use super::*;

extern "C" {
    #[link_name = "libspa_rs_format_audio_raw_parse"]
    pub fn spa_format_audio_raw_parse(
        format: *const spa_pod,
        info: *mut spa_audio_info_raw,
    ) -> c_int;

    #[link_name = "libspa_rs_format_audio_dsp_parse"]
    pub fn spa_format_audio_dsp_parse(
        format: *const spa_pod,
        info: *mut spa_audio_info_dsp,
    ) -> c_int;

    #[link_name = "libspa_rs_format_audio_iec958_parse"]
    pub fn spa_format_audio_iec958_parse(
        format: *const spa_pod,
        info: *mut spa_audio_info_iec958,
    ) -> c_int;

    #[link_name = "libspa_rs_format_audio_dsd_parse"]
    pub fn spa_format_audio_dsd_parse(
        format: *const spa_pod,
        info: *mut spa_audio_info_dsd,
    ) -> c_int;

    #[link_name = "libspa_rs_format_audio_raw_build"]
    pub fn spa_format_audio_raw_build(
        builder: *mut spa_pod_builder,
        id: u32,
        info: *mut spa_audio_info_raw,
    ) -> *mut spa_pod;

    #[link_name = "libspa_rs_format_audio_dsp_build"]
    pub fn spa_format_audio_dsp_build(
        builder: *mut spa_pod_builder,
        id: u32,
        info: *mut spa_audio_info_dsp,
    ) -> *mut spa_pod;

    #[link_name = "libspa_rs_format_audio_iec958_build"]
    pub fn spa_format_audio_iec958_build(
        builder: *mut spa_pod_builder,
        id: u32,
        info: *mut spa_audio_info_iec958,
    ) -> *mut spa_pod;

    #[link_name = "libspa_rs_format_audio_dsd_build"]
    pub fn spa_format_audio_dsd_build(
        builder: *mut spa_pod_builder,
        id: u32,
        info: *mut spa_audio_info_dsd,
    ) -> *mut spa_pod;
}
