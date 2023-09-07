#include <spa/pod/builder.h>

void libspa_rs_pod_builder_get_state(
    struct spa_pod_builder *builder,
    struct spa_pod_builder_state *state
) {
    spa_pod_builder_get_state(builder, state);
}

void libspa_rs_pod_builder_set_callbacks(
    struct spa_pod_builder *builder,
    const struct spa_pod_builder_callbacks *callbacks,
    void *data
) {
	spa_pod_builder_set_callbacks(builder, callbacks, data);
}

void libspa_rs_pod_builder_reset(
    struct spa_pod_builder *builder,
    struct spa_pod_builder_state *state
) {
    spa_pod_builder_reset(builder, state);
}

void libspa_rs_pod_builder_init(
    struct spa_pod_builder *builder,
    void *data,
    uint32_t size
) {
	spa_pod_builder_init(builder, data, size);
}

struct spa_pod *libspa_rs_pod_builder_deref(
    struct spa_pod_builder *builder,
    uint32_t offset
) {
	return spa_pod_builder_deref(builder, offset);
}

struct spa_pod *libspa_rs_pod_builder_frame(
    struct spa_pod_builder *builder,
    struct spa_pod_frame *frame)
{
	return spa_pod_builder_frame(builder, frame);
}

void libspa_rs_pod_builder_push(
    struct spa_pod_builder *builder,
    struct spa_pod_frame *frame,
    const struct spa_pod *pod,
    uint32_t offset
) {
    spa_pod_builder_push(builder, frame, pod, offset);
}

int libspa_rs_pod_builder_raw(
    struct spa_pod_builder *builder,
    const void *data,
    uint32_t size
) {
    return spa_pod_builder_raw(builder, data, size);
}

int libspa_rs_pod_builder_pad(
    struct spa_pod_builder *builder,
    uint32_t size
) {
    return spa_pod_builder_pad(builder, size);
}

int libspa_rs_pod_builder_raw_padded(
    struct spa_pod_builder *builder,
    const void *data,
    uint32_t size)
{
    return spa_pod_builder_raw_padded(builder, data, size);
}

void *libspa_rs_pod_builder_pop(
    struct spa_pod_builder *builder,
    struct spa_pod_frame *frame
) {
    return spa_pod_builder_pop(builder, frame);
}

int libspa_rs_pod_builder_primitive(
    struct spa_pod_builder *builder,
    const struct spa_pod *p
) {
    return spa_pod_builder_primitive(builder, p);
}

int libspa_rs_pod_builder_none(struct spa_pod_builder *builder)
{
	return spa_pod_builder_none(builder);
}

int libspa_rs_pod_builder_child(
    struct spa_pod_builder *builder,
    uint32_t size,
    uint32_t type
) {
	return spa_pod_builder_child(builder, size, type);
}

int libspa_rs_pod_builder_bool(struct spa_pod_builder *builder, bool val)
{
	return spa_pod_builder_bool(builder, val);
}

int libspa_rs_pod_builder_id(struct spa_pod_builder *builder, uint32_t val)
{
	return spa_pod_builder_id(builder, val);
}

int libspa_rs_pod_builder_int(struct spa_pod_builder *builder, int32_t val)
{
	return spa_pod_builder_int(builder, val);
}

int libspa_rs_pod_builder_long(struct spa_pod_builder *builder, int64_t val)
{
	return spa_pod_builder_long(builder, val);
}

int libspa_rs_pod_builder_float(struct spa_pod_builder *builder, float val)
{
	return spa_pod_builder_float(builder, val);
}

int libspa_rs_pod_builder_double(struct spa_pod_builder *builder, double val)
{
	return spa_pod_builder_double(builder, val);
}

int libspa_rs_pod_builder_write_string(
    struct spa_pod_builder *builder,
    const char *str,
    uint32_t len
) {
	return spa_pod_builder_write_string(builder, str, len);
}

int libspa_rs_pod_builder_string_len(
    struct spa_pod_builder *builder,
    const char *str,
    uint32_t len
) {
	return spa_pod_builder_string_len(builder, str, len);
}

int libspa_rs_pod_builder_string(
    struct spa_pod_builder *builder,
    const char *str
) {
	return spa_pod_builder_string(builder, str);
}

int libspa_rs_pod_builder_bytes(
    struct spa_pod_builder *builder,
    const void *bytes,
    uint32_t len
) {
	return spa_pod_builder_bytes(builder, bytes, len);
}

void *libspa_rs_pod_builder_reserve_bytes(
    struct spa_pod_builder *builder,
    uint32_t len
) {
	return spa_pod_builder_reserve_bytes(builder, len);
}

int libspa_rs_pod_builder_pointer(
    struct spa_pod_builder *builder,
    uint32_t type,
    const void *val)
{
	return spa_pod_builder_pointer(builder, type, val);
}

int libspa_rs_pod_builder_fd(struct spa_pod_builder *builder, int64_t fd)
{
	return spa_pod_builder_fd(builder, fd);
}

int libspa_rs_pod_builder_rectangle(
    struct spa_pod_builder *builder,
    uint32_t width,
    uint32_t height)
{
	return spa_pod_builder_rectangle(builder, width, height);
}

int libspa_rs_pod_builder_fraction(
    struct spa_pod_builder *builder,
    uint32_t num,
    uint32_t denom)
{
	return spa_pod_builder_fraction(builder, num, denom);
}

int libspa_rs_pod_builder_push_array(
    struct spa_pod_builder *builder,
    struct spa_pod_frame *frame
) {
	return spa_pod_builder_push_array(builder, frame);
}

int libspa_rs_pod_builder_array(
    struct spa_pod_builder *builder,
    uint32_t child_size,
    uint32_t child_type,
    uint32_t n_elems,
    const void *elems
) {
	return spa_pod_builder_array(builder, child_size, child_type, n_elems, elems);
}

int libspa_rs_pod_builder_push_choice(
    struct spa_pod_builder *builder,
    struct spa_pod_frame *frame,
    uint32_t type,
    uint32_t flags
) {
	return spa_pod_builder_push_choice(builder, frame, type, flags);
}

int libspa_rs_pod_builder_push_struct(
    struct spa_pod_builder *builder,
    struct spa_pod_frame *frame
) {
	return spa_pod_builder_push_struct(builder, frame);
}

int libspa_rs_pod_builder_push_object(
    struct spa_pod_builder *builder,
    struct spa_pod_frame *frame,
    uint32_t type,
    uint32_t id)
{
	return spa_pod_builder_push_object(builder, frame, type, id);
}

int libspa_rs_pod_builder_prop(
    struct spa_pod_builder *builder,
    uint32_t key,
    uint32_t flags)
{
	return spa_pod_builder_prop(builder, key, flags);
}

int libspa_rs_pod_builder_push_sequence(
    struct spa_pod_builder *builder,
    struct spa_pod_frame *frame,
    uint32_t unit)
{
	return spa_pod_builder_push_sequence(builder, frame, unit);
}

uint32_t libspa_rs_pod_builder_control(
    struct spa_pod_builder *builder,
    uint32_t offset,
    uint32_t type)
{
	return spa_pod_builder_control(builder, offset, type);
}

uint32_t libspa_rs_choice_from_id(char id)
{
    return spa_choice_from_id(id);
}

struct spa_pod *libspa_rs_pod_copy(const struct spa_pod *pod)
{
	return spa_pod_copy(pod);
}
