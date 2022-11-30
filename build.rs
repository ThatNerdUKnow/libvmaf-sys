use meson_next;
use std::env;
use std::fs::canonicalize;
use std::path::PathBuf;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let build_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("build");
    let lib_dir = build_dir.join("src");

    let build_dir_str = build_dir.to_str().unwrap();
    let lib_dir_str = lib_dir.to_str().unwrap();

    meson_next::build("vmaf/libvmaf", build_dir_str);

    println!("cargo:rustc-link-lib=static=vmaf");
    println!("cargo:rustc-link-search=native={lib_dir_str}");
    println!("cargo:rustc-flags=-l dylib=stdc++");

    // Path to vendor header files
    let headers_dir = PathBuf::from("vmaf/libvmaf/include");
    let headers_dir_canonical = canonicalize(headers_dir).unwrap();
    let include_path = headers_dir_canonical.to_str().unwrap();

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
