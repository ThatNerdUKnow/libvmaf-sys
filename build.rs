use meson_next;
use meson_next::config::Config;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

fn main() {
    let build_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("build");
    let lib_dir = build_dir.join("src");

    let build_dir_str = build_dir.to_str().unwrap();
    let lib_dir_str = lib_dir.to_str().unwrap();

    #[allow(unused_mut)]
    let mut meson_options = HashMap::<&str, &str>::new();

    #[cfg(feature = "avx512")]
    meson_options.insert("enable_avx512", "True");

    #[cfg(feature = "float")]
    meson_options.insert("enable_float", "True");

    let config: Config = Config::new().options(meson_options);

    #[cfg(target_os = "windows")]
    let config = {
        let native_file = canonicalize(Path::new("native-gcc-g++.ini")).unwrap();
        config.native_file(native_file);
    };

    println!("Build");

    meson_next::build("vmaf/libvmaf", build_dir_str, config);

    println!("cargo:rustc-link-lib=static=vmaf");
    println!("cargo:rustc-link-search=native={lib_dir_str}");
    println!("cargo:rustc-flags=-l dylib=stdc++");

    // Path to vendor header files
    let headers_dir = PathBuf::from("vmaf/libvmaf/include");
    let include_path = headers_dir.to_str().unwrap();

    println!("Bindgen");
    // Generate bindings to libvmaf using rust-bindgen
    let bindings = bindgen::Builder::default()
        .header("vmaf/libvmaf/include/libvmaf/libvmaf.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .clang_arg(format!("-I{include_path}"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to build directory
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
