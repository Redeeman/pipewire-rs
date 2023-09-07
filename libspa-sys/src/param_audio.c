#include <spa/param/audio/format-utils.h>

int libspa_rs_format_audio_raw_parse(const struct spa_pod* format, struct spa_audio_info_raw* info) {
    return spa_format_audio_raw_parse(format, info);
}

int libspa_rs_format_audio_dsp_parse(const struct spa_pod* format, struct spa_audio_info_dsp* info) {
    return spa_format_audio_dsp_parse(format, info);
}

int libspa_rs_format_audio_iec958_parse(const struct spa_pod* format, struct spa_audio_info_iec958* info) {
    return spa_format_audio_iec958_parse(format, info);
}

int libspa_rs_format_audio_dsd_parse(const struct spa_pod* format, struct spa_audio_info_dsd* info) {
    return spa_format_audio_dsd_parse(format, info);
}

struct spa_pod* libspa_rs_format_audio_raw_build(struct spa_pod_builder* builder, uint32_t id, struct spa_audio_info_raw* info) {
    return spa_format_audio_raw_build(builder, id, info);
}

struct spa_pod* libspa_rs_format_audio_dsp_build(struct spa_pod_builder* builder, uint32_t id, struct spa_audio_info_dsp* info) {
    return spa_format_audio_dsp_build(builder, id, info);
}

struct spa_pod* libspa_rs_format_audio_iec958_build(struct spa_pod_builder* builder, uint32_t id, struct spa_audio_info_iec958* info) {
    return spa_format_audio_iec958_build(builder, id, info);
}

struct spa_pod* libspa_rs_format_audio_dsd_build(struct spa_pod_builder* builder, uint32_t id, struct spa_audio_info_dsd* info) {
    return spa_format_audio_dsd_build(builder, id, info);
}
