#include <stdlib.h>
#include <stdint.h>
#include <vector>

namespace fpng
{
    struct buffer
    {
        std::vector<uint8_t> internal;
    };

    void buffer_create(buffer &buf);
    const uint8_t *buffer_pointer(const buffer &buf);
    const uint64_t buffer_size(const buffer &buf);
    void buffer_dispose(buffer &buf);

    bool fpng_encode_image_to_buffer(const void *pImage, uint32_t w, uint32_t h, uint32_t num_chans, buffer &buf, uint32_t flags = 0);
    int fpng_decode_buffer(const void *pImage, uint32_t image_size, buffer &buf, uint32_t &width, uint32_t &height, uint32_t &channels_in_file, uint32_t desired_channels);
}