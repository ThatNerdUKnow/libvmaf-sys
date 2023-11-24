use bindgen::CargoCallbacks;
use meson_next;
use meson_next::config::Config;
use std::collections::HashMap;
use std::env;
use std::fs::canonicalize;
use std::path::{Path, PathBuf};

fn main() {
    #[cfg(feature = "build")]
    let build_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("build");

    /*
    #[cfg(target_os = "windows")]
    let config = {
        let native_file = canonicalize(Path::new("native-gcc-g++.ini")).unwrap();
        config.native_file(native_file)
    };*/

    #[cfg(feature = "build")]
    build_lib(build_dir);

    println!("cargo:rustc-link-lib=static=vmaf");

    // Path to vendor header files
    #[cfg(feature = "build")]
    let include_path = PathBuf::from("vmaf/libvmaf/include").to_str().unwrap();

    println!("Bindgen");
    // Generate bindings to libvmaf using rust-bindgen
    let builder = bindgen::Builder::default()
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .parse_callbacks(Box::new(CargoCallbacks::new()));

    #[cfg(feature = "build")]
    let builder = builder
        .clang_arg(format!("-I{include_path}"))
        .header("vmaf/libvmaf/include/libvmaf/libvmaf.h");

    let lib = pkg_config::Config::new().probe("libvmaf").unwrap();

    let include = lib
        .include_paths
        .iter()
        .map(|i| format!("-I{}", i.to_string_lossy()));

    let builder = builder.clang_args(include);

    let builder = builder.header("wrapper.h");

    let bindings = builder
        .allowlist_item("[Vv]maf\\w*")
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to build directory
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(feature = "build")]
fn build_lib(build_dir: PathBuf) {
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

    println!("Build");
    println!("Directory: {build_dir_str}");

    meson_next::build("vmaf/libvmaf", build_dir_str, config);
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-link-search=native={lib_dir_str}");
}
