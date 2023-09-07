#include <spa/pod/parser.h>

void libspa_rs_pod_parser_init(
    struct spa_pod_parser* parser,
	const void* data,
    uint32_t size
) {
    spa_pod_parser_init(parser, data, size);
}

void libspa_rs_pod_parser_pod(struct spa_pod_parser* parser, const struct spa_pod* pod) {
    spa_pod_parser_pod(parser, pod);
}

void libspa_rs_pod_parser_get_state(struct spa_pod_parser *parser, struct spa_pod_parser_state *state) {
    spa_pod_parser_get_state(parser, state);
}

void libspa_rs_pod_parser_reset(struct spa_pod_parser* parser, struct spa_pod_parser_state* state) {
    spa_pod_parser_reset(parser, state);
}

struct spa_pod* libspa_rs_pod_parser_deref(
    struct spa_pod_parser* parser,
    uint32_t offset,
    uint32_t size
) {
    return spa_pod_parser_deref(parser, offset, size);
}

struct spa_pod* libspa_rs_pod_parser_frame(struct spa_pod_parser* parser, struct spa_pod_frame* frame) {
    return spa_pod_parser_frame(parser, frame);
}

void libspa_rs_pod_parser_push(
    struct spa_pod_parser* parser,
    struct spa_pod_frame* frame,
    const struct spa_pod* pod,
    uint32_t offset
) {
    spa_pod_parser_push(parser, frame, pod, offset);
}

struct spa_pod* libspa_rs_pod_parser_current(struct spa_pod_parser* parser) {
    return spa_pod_parser_current(parser);
}

void libspa_rs_pod_parser_advance(struct spa_pod_parser* parser, const struct spa_pod* pod) {
    spa_pod_parser_advance(parser, pod);
}

struct spa_pod* libspa_rs_pod_parser_next(struct spa_pod_parser* parser) {
    return spa_pod_parser_next(parser);
}

int libspa_rs_pod_parser_pop(struct spa_pod_parser* parser, struct spa_pod_frame* frame) {
    return spa_pod_parser_pop(parser, frame);
}

int libspa_rs_pod_parser_get_bool(struct spa_pod_parser* parser, bool* value) {
    return spa_pod_parser_get_bool(parser, value);
}

int libspa_rs_pod_parser_get_id(struct spa_pod_parser* parser, uint32_t* value) {
    return spa_pod_parser_get_id(parser, value);
}

int libspa_rs_pod_parser_get_int(struct spa_pod_parser* parser, int32_t* value) {
    return spa_pod_parser_get_int(parser, value);
}

int libspa_rs_pod_parser_get_long(struct spa_pod_parser* parser, int64_t* value) {
    return spa_pod_parser_get_long(parser, value);
}

int libspa_rs_pod_parser_get_float(struct spa_pod_parser* parser, float* value) {
    return spa_pod_parser_get_float(parser, value);
}

int libspa_rs_pod_parser_get_double(struct spa_pod_parser* parser, double* value) {
    return spa_pod_parser_get_double(parser, value);
}

int libspa_rs_pod_parser_get_string(struct spa_pod_parser* parser, const char** value) {
    return spa_pod_parser_get_string(parser, value);
}

int libspa_rs_pod_parser_get_bytes(struct spa_pod_parser* parser, const void** value, uint32_t* len) {
    return spa_pod_parser_get_bytes(parser, value, len);
}

int libspa_rs_pod_parser_get_pointer(struct spa_pod_parser* parser, uint32_t* type, const void** value) {
    return spa_pod_parser_get_pointer(parser, type, value);
}

int libspa_rs_pod_parser_get_fd(struct spa_pod_parser* parser, int64_t* value) {
    return spa_pod_parser_get_fd(parser, value);
}

int libspa_rs_pod_parser_get_rectangle(struct spa_pod_parser* parser, struct spa_rectangle* value) {
    return spa_pod_parser_get_rectangle(parser, value);
}

int libspa_rs_pod_parser_get_fraction(struct spa_pod_parser* parser, struct spa_fraction* value) {
    return spa_pod_parser_get_fraction(parser, value);
}

int libspa_rs_pod_parser_get_pod(struct spa_pod_parser* parser, struct spa_pod** value) {
    return spa_pod_parser_get_pod(parser, value);
}

int libspa_rs_pod_parser_push_struct(struct spa_pod_parser* parser, struct spa_pod_frame* frame) {
    return spa_pod_parser_push_struct(parser, frame);
}

int libspa_rs_pod_parser_push_object(
    struct spa_pod_parser* parser,
    struct spa_pod_frame* frame,
    uint32_t type,
    uint32_t* id
) {
    return spa_pod_parser_push_object(parser, frame, type, id);
}
