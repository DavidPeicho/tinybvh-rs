fn main() {
    cxx_build::bridge("src/cxx_ffi.rs")
        .file("ffi/src/tinybvh.cpp")
        .std("c++20")
        .compile("tinybvh");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=tinybvh/tiny_bvh.h");
    println!("cargo:rerun-if-changed=src/tiny_bvh.h");
    println!("cargo:rerun-if-changed=src/tiny_bvh.cpp");
}
