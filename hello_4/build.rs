fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/calc.cpp")
        .flag_if_supported("-std=c++17")
        .compile("calc");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/calc.cpp");
    println!("cargo:rerun-if-changed=src/calc.h");
}