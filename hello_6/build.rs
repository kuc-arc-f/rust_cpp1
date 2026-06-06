fn main() {
    cxx_build::bridge("src/main.rs")
        .file("include/my_lib.hpp")
        .flag_if_supported("-std=c++17")
        .compile("my_lib");

    //LIB    
    println!("cargo:rustc-link-lib=curl");
    // build.rs 再実行条件
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/my_lib.hpp");
}