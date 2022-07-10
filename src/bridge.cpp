#include "fpng.h"
#include "bridge.h"

namespace fpng
{
    void create_buffer(buffer &buf)
    {
        buf = std::move(buffer());
    }

    void dispose_buffer(buffer &buf)
    {
        std::move(buf);
    }

    bool fpng_encode_image_to_buffer(const void *pImage, uint32_t w, uint32_t h, uint32_t num_chans, buffer &buf, uint32_t flags)
    {
        bool result = fpng_encode_image_to_memory(pImage, w, h, num_chans, buf.internal, flags);
        buf.pointer = (void *)&*(buf.internal.begin());
        buf.size = (uint64_t)buf.internal.size();
        return result;
    }

    int fpng_decode_buffer(const void *pImage, uint32_t image_size, buffer &buf, uint32_t &width, uint32_t &height, uint32_t &channels_in_file, uint32_t desired_channels)
    {
        int result = fpng_decode_memory(pImage, image_size, buf.internal, width, height, channels_in_file, desired_channels);
        buf.pointer = (void *)&*(buf.internal.begin());
        buf.size = (uint64_t)buf.internal.size();
        return result;
    }
}