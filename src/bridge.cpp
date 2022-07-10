#include "fpng.h"
#include "bridge.h"

namespace fpng
{

    void buffer_create(buffer &buf)
    {
        buf = std::move(buffer());
    }

    const uint8_t *buffer_pointer(const buffer &buf)
    {
        return &*buf.internal.begin();
    }

    const uint64_t buffer_size(const buffer &buf)
    {
        return (uint64_t)buf.internal.size();
    }

    void buffer_dispose(buffer &buf)
    {
        std::move(buf);
    }

    bool fpng_encode_image_to_buffer(const void *pImage, uint32_t w, uint32_t h, uint32_t num_chans, buffer &buf, uint32_t flags)
    {
        return fpng_encode_image_to_memory(pImage, w, h, num_chans, buf.internal, flags);
    }

    int fpng_decode_buffer(const void *pImage, uint32_t image_size, buffer &buf, uint32_t &width, uint32_t &height, uint32_t &channels_in_file, uint32_t desired_channels)
    {
        return fpng_decode_memory(pImage, image_size, buf.internal, width, height, channels_in_file, desired_channels);
    }
}