#include <spa/param/video/format-utils.h>

int libspa_rs_format_video_raw_parse(const struct spa_pod* format, struct spa_video_info_raw* info) {
    return spa_format_video_raw_parse(format, info);
}

int libspa_rs_format_video_dsp_parse(const struct spa_pod* format, struct spa_video_info_dsp* info) {
    return spa_format_video_dsp_parse(format, info);
}

int libspa_rs_format_video_h264_parse(const struct spa_pod* format, struct spa_video_info_h264* info) {
    return spa_format_video_h264_parse(format, info);
}

int libspa_rs_format_video_mjpg_parse(const struct spa_pod* format, struct spa_video_info_mjpg* info) {
    return spa_format_video_mjpg_parse(format, info);
}

struct spa_pod* libspa_rs_format_video_raw_build(struct spa_pod_builder* builder, uint32_t id, struct spa_video_info_raw* info) {
    return spa_format_video_raw_build(builder, id, info);
}

struct spa_pod* libspa_rs_format_video_dsp_build(struct spa_pod_builder* builder, uint32_t id, struct spa_video_info_dsp* info) {
    return spa_format_video_dsp_build(builder, id, info);
}

struct spa_pod* libspa_rs_format_video_h264_build(struct spa_pod_builder* builder, uint32_t id, struct spa_video_info_h264* info) {
    return spa_format_video_h264_build(builder, id, info);
}

struct spa_pod* libspa_rs_format_video_mjpg_build(struct spa_pod_builder* builder, uint32_t id, struct spa_video_info_mjpg* info) {
    return spa_format_video_mjpg_build(builder, id, info);
}
