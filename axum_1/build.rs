fn main() {
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=dylib=sample");
    println!("cargo:rustc-env=LD_LIBRARY_PATH=.");
}