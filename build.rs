extern crate meson;
use std::env;
use std::path::PathBuf;

fn main(){
    let build_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    build_path.join("build");
    let build_path = build_path.to_str().unwrap();

    println!("cargo:rustc-link-lib=libvmaf");
    println!("cargo:rustc-link-search=native={build_path}");
    meson::build("vmaf/libvmaf",build_path);
}