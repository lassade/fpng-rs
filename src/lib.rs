#[cfg(not(feature = "internal-bindgen-on-build"))]
#[allow(non_camel_case_types)]
pub mod sys {
    include!("fpng.rs");
    include!("bridge.rs");
}

#[cfg(not(feature = "internal-bindgen-on-build"))]
pub use bindings::*;

#[cfg(not(feature = "internal-bindgen-on-build"))]
pub mod bindings {
    use thiserror::Error;

    use super::sys;

    pub struct Buffer {
        internal: sys::buffer,
    }

    impl Default for Buffer {
        fn default() -> Self {
            unsafe {
                let mut internal = core::mem::MaybeUninit::zeroed().assume_init();
                sys::buffer_create(&mut internal as *mut _);
                Self { internal }
            }
        }
    }

    impl Buffer {
        pub fn bytes(&self) -> &[u8] {
            unsafe {
                core::slice::from_raw_parts(
                    sys::buffer_pointer(&self.internal as *const _),
                    sys::buffer_size(&self.internal as *const _) as _,
                )
            }
        }
    }

    impl Drop for Buffer {
        fn drop(&mut self) {
            unsafe { sys::buffer_dispose(&mut self.internal as *mut _) }
        }
    }

    /// Call [`Png::detect_cpu()`] once before using so it can detect if the CPU supports SSE 4.1+pclmul (for fast CRC-32 and Adler32).
    /// Otherwise, it'll always use the slower scalar fallbacks.
    pub struct FPng {
        width: u32,
        height: u32,
        channels: u32,
        buffer: Buffer,
    }

    #[derive(Debug, Error)]
    pub enum FPngCreationError {
        //Success = sys::FPNG_DECODE_SUCCESS as _,
        #[error("file is a valid PNG file, but it wasn't written by FPNG so you should try decoding it with a general purpose PNG decoder")]
        NotFPng = sys::FPNG_DECODE_NOT_FPNG as _,
        #[error("invalid function parameter")]
        InvalidArg = sys::FPNG_DECODE_INVALID_ARG as _,
        #[error("file cannot be a PNG file")]
        FailedNotPng = sys::FPNG_DECODE_FAILED_NOT_PNG as _,
        #[error("a chunk CRC32 check failed, file is likely corrupted or not PNG")]
        FailedHeaderCrc32 = sys::FPNG_DECODE_FAILED_HEADER_CRC32 as _,
        #[error("invalid image dimensions in IHDR chunk (0 or too large)")]
        FailedInvalidDimensions = sys::FPNG_DECODE_FAILED_INVALID_DIMENSIONS as _,
        #[error("decoding the file fully into memory would likely require too much memory (only on 32bpp builds)")]
        FailedDimensionsTooLarge = sys::FPNG_DECODE_FAILED_DIMENSIONS_TOO_LARGE as _,
        #[error("failed while parsing the chunk headers, or file is corrupted")]
        FailedChunkParsing = sys::FPNG_DECODE_FAILED_CHUNK_PARSING as _,
        #[error(
            "IDAT data length is too small and cannot be valid, file is either corrupted or it's a bug"
        )]
        FailedInvalidIdat = sys::FPNG_DECODE_FAILED_INVALID_IDAT as _,
        // FileOpenFailed = sys::FPNG_DECODE_FILE_OPEN_FAILED,
        // FileTooLarge = sys::FPNG_DECODE_FILE_TOO_LARGE,
        // FileReadFailed = sys::FPNG_DECODE_FILE_READ_FAILED,
        // FileSeekFailed = sys::FPNG_DECODE_FILE_SEEK_FAILED,
    }

    bitflags::bitflags! {
        pub struct FPngEncodeFlags: u32 {
            /// Enables computing custom Huffman tables for each file, instead of using the custom global tables.
            /// Results in roughly 6% smaller files on average, but compression is around 40% slower.
            const FPNG_ENCODE_SLOWER = sys::FPNG_ENCODE_SLOWER as u32;
            /// Only use raw Deflate blocks (no compression at all). Intended for testing.
            const FPNG_FORCE_UNCOMPRESSED = sys::FPNG_FORCE_UNCOMPRESSED as u32;
        }
    }

    impl FPng {
        pub const fn width(&self) -> u32 {
            self.width
        }

        pub const fn height(&self) -> u32 {
            self.height
        }

        pub const fn channels(&self) -> u32 {
            self.channels
        }

        pub fn bytes(&self) -> &[u8] {
            self.buffer.bytes()
        }

        /// Call once to identify if the processor supports SSE.
        pub fn init() {
            unsafe {
                sys::fpng_init();
            }
        }

        /// Create a png from bytes
        pub fn from_bytes(bytes: &[u8], desired_channels: u32) -> Result<FPng, FPngCreationError> {
            let mut png = FPng {
                width: 0,
                height: 0,
                channels: desired_channels,
                buffer: Buffer::default(),
            };

            unsafe {
                let mut chnnels_in_file: u32 = 0;

                let flags = sys::fpng_decode_buffer(
                    bytes.as_ptr() as *const _,
                    bytes.len() as _,
                    &mut png.buffer.internal as *mut _,
                    &mut png.width as *mut _,
                    &mut png.height as *mut _,
                    &mut chnnels_in_file as *mut _,
                    desired_channels,
                );

                if flags != sys::FPNG_DECODE_SUCCESS {
                    return Err(core::mem::transmute(flags as u8));
                }
            }

            Ok(png)
        }

        pub fn encode(
            bytes: &[u8],
            width: u32,
            height: u32,
            channels: u32,
            flags: FPngEncodeFlags,
        ) -> Buffer {
            let mut buffer = Buffer::default();

            unsafe {
                sys::fpng_encode_image_to_buffer(
                    bytes.as_ptr() as *const _,
                    width,
                    height,
                    channels,
                    &mut buffer.internal as *mut _,
                    flags.bits(),
                );
            }

            buffer
        }
    }
}

#[cfg(not(feature = "internal-bindgen-on-build"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load() {
        FPng::init();
        let png =
            FPng::from_bytes(include_bytes!("../fpng/example.png"), 4).expect("failed to load png");
        assert_eq!(png.width(), 687);
        assert_eq!(png.height(), 1012);
        assert_eq!(png.channels(), 4);
        assert_eq!(
            png.width() * png.height() * png.channels(),
            png.bytes().len() as _
        );
    }
}
