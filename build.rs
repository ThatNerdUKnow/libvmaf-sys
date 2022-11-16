extern crate meson;
use std::env;
use std::fs::canonicalize;
use std::path::PathBuf;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let build_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    _ = build_path.join("build");
    let build_path = build_path.to_str().unwrap();

    println!("cargo:rustc-link-lib=libvmaf");
    println!("cargo:rustc-link-search=native={build_path}");
    meson::build("vmaf/libvmaf", build_path);

    let include_path = PathBuf::from("vmaf/libvmaf/include");
    let binding = canonicalize(include_path).unwrap();
    let include_path = binding.to_str().unwrap();
    //println!("cargo:include=vmaf/libvmaf/include");
    let bindings = bindgen::Builder::default()
    .header("vmaf/libvmaf/include/libvmaf/libvmaf.h")
    .clang_arg(format!("-I{include_path}"))
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
