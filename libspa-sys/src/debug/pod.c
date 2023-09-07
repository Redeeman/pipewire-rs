#include <spa/debug/pod.h>

int libspa_rs_debugc_pod_value(struct spa_debug_context *ctx, int indent, const struct spa_type_info *info,
		uint32_t type, void *body, uint32_t size)
{
    return spa_debugc_pod_value(ctx, indent, info, type, body, size);
}

int libspa_rs_debugc_pod(struct spa_debug_context *ctx, int indent,
		const struct spa_type_info *info, const struct spa_pod *pod)
{
    return spa_debugc_pod(ctx, indent, info, pod);
}

int libspa_rs_debug_pod_value(int indent, const struct spa_type_info *info,
		uint32_t type, void *body, uint32_t size)
{
    return spa_debug_pod_value(indent, info, type, body, size);
}

int libspa_rs_debug_pod(int indent, const struct spa_type_info *info, const struct spa_pod *pod)
{
	return spa_debug_pod(indent, info, pod);
}
