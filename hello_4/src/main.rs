#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("hello_4/src/calc.h");

        fn add(a: i32, b: i32) -> i32;
    }
}

fn main() {
    let result = ffi::add(10, 20);

    println!("result = {}", result);
}