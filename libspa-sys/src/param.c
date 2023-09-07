#include <spa/param/format-utils.h>

int libspa_rs_format_parse(const struct spa_pod* format, uint32_t* media_type, uint32_t* media_subtype) {
    return spa_format_parse(format, media_type, media_subtype);
}
