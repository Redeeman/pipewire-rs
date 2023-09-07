#include <spa/pod/iter.h>

// TODO: spa_pod_is_inside, spa_pod_next, spa_pod_prop_first, spa_pod_prop_is_inside, spa_pod_prop_next,
//       spa_pod_control_first, spa_pod_control_is_inside, spa_pod_control_next, spa_pod_from_data

int libspa_rs_pod_is_none(const struct spa_pod *pod)
{
        return spa_pod_is_none(pod);
}

int libspa_rs_pod_is_bool(const struct spa_pod *pod)
{
        return spa_pod_is_bool(pod);
}

int libspa_rs_pod_get_bool(const struct spa_pod *pod, bool *value)
{
        return spa_pod_get_bool(pod, value);
}

int libspa_rs_pod_is_id(const struct spa_pod *pod)
{
        return spa_pod_is_id(pod);
}

int libspa_rs_pod_get_id(const struct spa_pod *pod, uint32_t *value)
{
        return spa_pod_get_id(pod, value);
}

int libspa_rs_pod_is_int(const struct spa_pod *pod)
{
        return spa_pod_is_int(pod);
}

int libspa_rs_pod_get_int(const struct spa_pod *pod, int32_t *value)
{
        return spa_pod_get_int(pod, value);
}

int libspa_rs_pod_is_long(const struct spa_pod *pod)
{
        return spa_pod_is_long(pod);
}

int libspa_rs_pod_get_long(const struct spa_pod *pod, int64_t *value)
{
        return spa_pod_get_long(pod, value);
}

int libspa_rs_pod_is_float(const struct spa_pod *pod)
{
        return libspa_rs_pod_is_float(pod);
}

int libspa_rs_pod_get_float(const struct spa_pod *pod, float *value)
{
        return spa_pod_get_float(pod, value);
}

int libspa_rs_pod_is_double(const struct spa_pod *pod)
{
        return spa_pod_is_double(pod);
}

int libspa_rs_pod_get_double(const struct spa_pod *pod, double *value)
{
        return spa_pod_get_double(pod, value);
}

int libspa_rs_pod_is_string(const struct spa_pod *pod)
{
        return spa_pod_is_string(pod);
}

int libspa_rs_pod_get_string(const struct spa_pod *pod, const char **value)
{
        return spa_pod_get_string(pod, value);
}

int libspa_rs_pod_is_bytes(const struct spa_pod *pod)
{
        return spa_pod_is_bytes(pod);
}

int libspa_rs_pod_get_bytes(const struct spa_pod *pod, const void **value, uint32_t *len)
{
        return spa_pod_get_bytes(pod, value);
}

int libspa_rs_pod_is_pointer(const struct spa_pod *pod)
{
        return spa_pod_is_pointer(pod);
}

int libspa_rs_pod_get_pointer(const struct spa_pod *pod, uint32_t *type, const void **value)
{
        return spa_pod_get_pointer(pod, value);
}

int libspa_rs_pod_is_fd(const struct spa_pod *pod)
{
        return spa_pod_is_fd(pod);
}

int libspa_rs_pod_get_fd(const struct spa_pod *pod, int64_t *value)
{
        return spa_pod_get_fd(pod, value);
}

int libspa_rs_pod_is_rectangle(const struct spa_pod *pod)
{
        return spa_pod_is_rectangle(pod);
}

int libspa_rs_pod_get_rectangle(const struct spa_pod *pod, struct spa_rectangle *value)
{
        return spa_pod_get_rectangle(pod, value);
}

int libspa_rs_pod_is_fraction(const struct spa_pod *pod)
{
        return spa_pod_is_fraction(pod);
}

int libspa_rs_pod_get_fraction(const struct spa_pod *pod, struct spa_fraction *value)
{
        return spa_pod_get_id(pod, value);
}

int libspa_rs_pod_is_bitmap(const struct spa_pod *pod)
{
        return spa_pod_is_bitmap(pod);
}

int libspa_rs_pod_is_array(const struct spa_pod *pod)
{
        return spa_pod_is_array(pod);
}

// TODO: spa_pod_get_array
// TODO: spa_pod_copy_array

int libspa_rs_pod_is_choice(const struct spa_pod *pod)
{
        return spa_pod_is_choice(pod);
}

// TODO: spa_pod_get_values

int libspa_rs_pod_is_struct(const struct spa_pod *pod)
{
        return spa_pod_is_struct(pod);
}

int libspa_rs_pod_is_object(const struct spa_pod *pod)
{
        return spa_pod_is_double(pod);
}

// TODO: spa_pod_is_object_type, spa_pod_is_object_id

int libspa_rs_pod_is_sequence(const struct spa_pod *pod)
{
        return spa_pod_is_double(pod);
}

// TODO: spa_pod_object_find_prop, spa_pod_find_prop, spa_pod_object_fixate, spa_pod_fixate, spa_pod_object_is_fixated, spa_pod_is_fixated
