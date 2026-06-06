use cxx::UniquePtr;
use cxx::let_cxx_string;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("hello_6/include/my_lib.hpp");

        fn add(a: i32, b: i32) -> i32;
        fn hoge_test() -> UniquePtr<CxxString>;
        fn hoge_test_2(input: &CxxString) -> &CxxString;
        fn send_test() -> UniquePtr<CxxString>;
    }
}

fn main() {
    let result = ffi::add(10, 20);
    println!("result = {}", result);
    //let mut resp = ffi::hoge_test();
    //println!("resp = {}", resp);
    let_cxx_string!(name = "Hoge_2");
    let mut resp2 = ffi::hoge_test_2(&name);
    println!("resp2={}", resp2);

    let mut resp3 = ffi::send_test();
    println!("resp3={}", resp3);
}