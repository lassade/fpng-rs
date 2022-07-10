fn main() {
    internal::run();
}

#[cfg(not(feature = "internal-bindgen-on-build"))]
mod internal {
    pub fn run() {
        // todo link cpp file
        cc::Build::new()
            .file("fpng/src/fpng.cpp")
            .define("FPNG_NO_STDIO", None)
            .include("fpng/src")
            .compile("fpng");

        cc::Build::new()
            .file("src/bridge.cpp")
            .define("FPNG_NO_STDIO", None)
            .include("fpng/src")
            .include("scr")
            .compile("bridge");
    }
}

#[cfg(feature = "internal-bindgen-on-build")]
mod internal {
    pub fn run() {
        bindgen::Builder::default()
            .header("fpng/src/fpng.h")
            .layout_tests(false)
            .prepend_enum_name(true)
            .disable_name_namespacing()
            .generate_comments(false)
            .allowlist_function("^(fpng).*")
            .allowlist_type("^(fpng).*")
            .allowlist_var("^(fpng).*")
            .blocklist_item(".*(fpng_decode_memory|fpng_encode_image_to_memory).*")
            .ctypes_prefix("cty")
            .allowlist_recursively(false)
            .generate_inline_functions(true)
            .rust_target(bindgen::RustTarget::Nightly)
            .use_core()
            .size_t_is_usize(true)
            .clang_args([
                "-x",
                "c++",
                "-std=c++17",
                "-fno-inline-functions",
                "-Ifpng\\src\\",
                "-DFPNG_NO_STDIO",
            ])
            .generate()
            .expect("unable to generate bindings")
            .write_to_file("src/fpng.rs")
            .expect("couldn't write bindings in `src/fpng.rs`");

        bindgen::Builder::default()
            .header("src/bridge.h")
            .layout_tests(false)
            .prepend_enum_name(true)
            .disable_name_namespacing()
            .generate_comments(false)
            .allowlist_function("^(fpng).*")
            .allowlist_type("^(fpng).*")
            .allowlist_var("^(fpng).*")
            .opaque_type("^(std).*")
            .ctypes_prefix("cty")
            .allowlist_recursively(false)
            .generate_inline_functions(true)
            .rust_target(bindgen::RustTarget::Nightly)
            .use_core()
            .size_t_is_usize(true)
            .clang_args([
                "-x",
                "c++",
                "-std=c++17",
                "-fno-inline-functions",
                "-Ifpng\\src\\",
                "-DFPNG_NO_STDIO",
            ])
            .generate()
            .expect("unable to generate bindings")
            .write_to_file("src/bridge.rs")
            .expect("couldn't write bindings in `src/bridge.rs`");
    }
}
