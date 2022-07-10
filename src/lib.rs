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

    /// Call [`Png::detect_cpu()`] once before using so it can detect if the CPU supports SSE 4.1+pclmul (for fast CRC-32 and Adler32).
    /// Otherwise, it'll always use the slower scalar fallbacks.
    pub struct FPng {
        width: u32,
        height: u32,
        channels: u32,
        buffer: sys::buffer,
    }

    #[derive(Debug, Error)]
    pub enum PngCreationError {
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

    impl Drop for FPng {
        fn drop(&mut self) {
            unsafe { sys::dispose_buffer(&mut self.buffer as *mut _) }
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
            unsafe {
                core::slice::from_raw_parts(self.buffer.pointer as *const _, self.buffer.size as _)
            }
        }

        pub fn rgba32(&self) -> &[u32] {
            assert!(self.channels == 4, "not a 4 chanel png");
            unsafe {
                core::slice::from_raw_parts(
                    self.buffer.pointer as *const _,
                    self.buffer.size as usize / core::mem::size_of::<u32>(),
                )
            }
        }

        /// Call once to identify if the processor supports SSE.
        pub fn detect_cpu() {
            unsafe {
                sys::fpng_init();
            }
        }

        /// Create a png from bytes
        pub fn from_bytes(bytes: &[u8], desired_channels: u32) -> Result<FPng, PngCreationError> {
            let mut png = FPng {
                width: 0,
                height: 0,
                channels: 0,
                buffer: unsafe { core::mem::MaybeUninit::zeroed().assume_init() },
            };

            unsafe {
                sys::create_buffer(&mut png.buffer as *mut _);
            }

            unsafe {
                let flags = sys::fpng_decode_buffer(
                    bytes.as_ptr() as *const _,
                    bytes.len() as _,
                    &mut png.buffer as *mut _,
                    &mut png.width as *mut _,
                    &mut png.height as *mut _,
                    &mut png.channels as *mut _,
                    desired_channels,
                );

                if flags != sys::FPNG_DECODE_SUCCESS {
                    return Err(core::mem::transmute(flags as u8));
                }
            }

            Ok(png)
        }
    }
}
