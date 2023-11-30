// Disable deprecation warning on CargoCallbacks import because Struct and Const "CargoCallbacks" have a name collision
// I pinky promise i'm using the right one
#[allow(deprecated)]
use bindgen::{Builder, CargoCallbacks};

use std::env;

use std::path::PathBuf;

fn main() {
    #[cfg(feature = "build")]
    build_lib();

    link_lib();

    println!("Bindgen");
    // Generate bindings to libvmaf using rust-bindgen
    let builder = bindgen::Builder::default()
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .allowlist_item("[Vv]maf\\w*")
        .parse_callbacks(Box::new(CargoCallbacks::new()));

    let builder = gen_bindings(builder);

    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write bindings to build directory
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

/// Set linker flags for required libraries
fn link_lib() {
    #[cfg(feature = "static")]
    let vmaf_link_style = "static";
    #[cfg(not(feature = "static"))]
    let vmaf_link_style = "dylib";

    #[cfg(target_os = "macos")]
    let cpp_stdlib = "c++";
    #[cfg(not(target_os = "macos"))]
    let cpp_stdlib = "stdc++";

    println!("cargo:rustc-link-lib={vmaf_link_style}=vmaf");

    // c++ standard library can only be linked to dynamically
    println!("cargo:rustc-link-lib=dylib={cpp_stdlib}");
}

#[cfg(feature = "build")]
fn build_lib() {
    use meson_next::config::Config;
    use std::{collections::HashMap, process::Command};

    // Pull vmaf git submodule
    let _git_submodule_update = Command::new("git")
        .args(["submodule", "update", "--recursive", "--init"])
        .status()
        .expect("Could not update vmaf git submodule");

    let build_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("build");
    let lib_dir = build_dir.join("src");
    let build_dir_str = build_dir.to_str().unwrap();
    let lib_dir_str = lib_dir.to_str().unwrap();

    let meson_options = HashMap::<&str, &str>::new();

    #[cfg(feature = "avx512")]
    let meson_options = meson_options.insert("enable_avx512", "True");

    #[cfg(feature = "float")]
    let meson_options = meson_options.insert("enable_float", "True");

    let config: Config = Config::new().options(meson_options);

    println!("Build");
    println!("Directory: {build_dir_str}");

    meson_next::build("vmaf/libvmaf", build_dir_str, config);
    println!("cargo:rustc-link-search=native={lib_dir_str}");
}

#[cfg(feature = "build")]
fn gen_bindings(builder: Builder) -> Builder {
    // Path to vendor header files
    let include_path = PathBuf::from("vmaf/libvmaf/include");
    let include_str = include_path
        .to_str()
        .expect("Could not format vmaf include directory string");

    builder
        .clang_arg(format!("-I{include_str}"))
        .header("vmaf/libvmaf/include/libvmaf/libvmaf.h")
}

#[cfg(not(feature = "build"))]
fn gen_bindings(builder: Builder) -> Builder {
    println!("cargo:rerun-if-changed=wrapper.h");
    let lib = pkg_config::Config::new()
        .probe("libvmaf")
        .expect("pkg-config can't find library libvmaf");

    let include = lib
        .include_paths
        .iter()
        .map(|i| format!("-I{}", i.to_string_lossy()));

    builder.clang_args(include).header("wrapper.h")
}
