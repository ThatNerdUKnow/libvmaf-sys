# About
raw Rust bindings for libvmaf 2.0 and above from Netflix

There are 2 ways to consume this library. The method used is determined by the `build` feature.

- Build feature enabled
    - builds and links to `libvmaf` from source
- Build feature disabled
    - uses locally installed `libvmaf` to generate bindings

## Requirements
- build feature enabled
    * meson
    * ninja
    * nasm
    * python
- build feature disabled
    * pkg-config
- Both
    * clang

## Building on Windows
Building this library on windows will follow the same requirements as `libvmaf`. You'll need MinGW64 with the following packages installed: 
- build feature enabled
    - mingw-w64-x86_64-nasm
    - mingw-w64-x86_64-gcc 
    - mingw-w64-x86_64-meson
    - mingw-w64-x86_64-ninja
- build feature disabled
    - mingw-w64-x86_64-vmaf
    - mingw-w64-x86_64-pkg-config
- Both
    - mingw-w64-x86_64-clang

Also, make sure that you add the MinGW bin directory to your PATH
By default this will be `C:\msys64\mingw64\bin`