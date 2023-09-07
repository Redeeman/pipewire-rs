#include <spa/utils/ringbuffer.h>

void libspa_rs_utils_ringbuffer_init(struct spa_ringbuffer *rbuf)
{
	spa_ringbuffer_init(rbuf);
}

void libspa_rs_utils_ringbuffer_set_avail(struct spa_ringbuffer *rbuf,
					  uint32_t size)
{
	spa_ringbuffer_set_avail(rbuf, size);
}

int32_t libspa_rs_utils_ringbuffer_get_read_index(struct spa_ringbuffer *rbuf,
						  uint32_t * index)
{
	return spa_ringbuffer_get_read_index(rbuf, index);
}

void libspa_rs_utils_ringbuffer_read_data(struct spa_ringbuffer *rbuf,
					  const void *buffer,
					  uint32_t size,
					  uint32_t offset,
					  void *data, uint32_t len)
{
	spa_ringbuffer_read_data(rbuf, buffer, size, offset, data, len);
}

void libspa_rs_utils_ringbuffer_read_update(struct spa_ringbuffer *rbuf,
					    int32_t index)
{
	spa_ringbuffer_read_update(rbuf, index);
}

int32_t libspa_rs_utils_ringbuffer_get_write_index(struct spa_ringbuffer *rbuf,
						   uint32_t * index)
{
	return spa_ringbuffer_get_write_index(rbuf, index);
}

void libspa_rs_utils_ringbuffer_write_data(struct spa_ringbuffer *rbuf,
					   void *buffer,
					   uint32_t size,
					   uint32_t offset,
					   const void *data, uint32_t len)
{
	spa_ringbuffer_write_data(rbuf, buffer, size, offset, data, len);
}

void libspa_rs_utils_ringbuffer_write_update(struct spa_ringbuffer *rbuf,
					     int32_t index)
{
	spa_ringbuffer_write_update(rbuf, index);
}
