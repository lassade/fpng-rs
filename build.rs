#[cfg(not(doc))]

fn main() {
    internal::run();
}

#[cfg(not(feature = "internal-bindgen-on-build"))]
mod internal {
    pub fn run() {
        let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
        std::fs::remove_dir_all(&out).unwrap();
        std::fs::create_dir(&out).unwrap();

        // todo link cpp file
        cc::Build::new()
            .file("fpng/src/fpng.cpp")
            .cpp(true)
            .define("FPNG_NO_STDIO", None)
            .include("fpng/src")
            .compile("fpng");

        cc::Build::new()
            .file("src/bridge.cpp")
            .cpp(true)
            .define("FPNG_NO_STDIO", None)
            .include("fpng/src")
            .include("src")
            .compile("bridge");

        // // Test that the `windows_registry` module will set PATH by looking for
        // // nmake which runs vanilla cl, and then also test it after we remove all
        // // the relevant env vars from our own process.
        // if target.contains("msvc") {
        //     let out = out.join("tmp");
        //     fs::create_dir(&out).unwrap();

        //     fs::remove_dir_all(&out).unwrap();
        //     fs::create_dir(&out).unwrap();

        //     env::remove_var("PATH");
        //     env::remove_var("VCINSTALLDIR");
        //     env::remove_var("INCLUDE");
        //     env::remove_var("LIB");
        //     println!("nmake 2");
        //     let status = cc::windows_registry::find(&target, "nmake.exe")
        //         .unwrap()
        //         .env_remove("MAKEFLAGS")
        //         .arg("/fsrc/NMakefile")
        //         .env("OUT_DIR", &out)
        //         .status()
        //         .unwrap();
        //     assert!(status.success());
        //     println!("cargo:rustc-link-lib=msvc");
        //     println!("cargo:rustc-link-search={}", out.display());
        // }
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
            .opaque_type("fpng::buffer")
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
