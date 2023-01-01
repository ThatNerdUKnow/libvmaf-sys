//! This crate is a simple bindgen wrapper library around Netflix's `libvmaf`  
//! You're probably looking for [libvmaf-rs](https://crates.io/crates/libvmaf-rs)


#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
