fn main() {
    cxx_build::bridge("src/main.rs")
        .file("include/my_lib.hpp")
        .flag_if_supported("-std=c++17")
        .compile("my_lib");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/my_lib.hpp");
}