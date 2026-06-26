use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;
use std::env;

extern "C" {
    fn add(a: i32, b: i32) -> i32;

    fn rag_add() -> *mut c_char;
    fn rag_search(input: *const c_char) -> *mut c_char;
    
    // メモリ解放
    fn free_string(ptr: *mut c_char);
    
}

/**
*
* @param
*
* @return
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cmd = String::new();
    if args.len() < 2 {
        println!("error: no argument");
        return;
    }    
    cmd = args[1].clone();
    println!("cmd={}", cmd);

    if cmd == "embed" {
        unsafe {
            let result_ptr = rag_add();
            if !result_ptr.is_null() {
                let result_cstr = CStr::from_ptr(result_ptr);
                let result_str = result_cstr.to_str().unwrap();
                println!("C++ returned: '{}'", result_str);
                free_string(result_ptr);
            }
        }
    }
    if cmd == "search" {
        let query = args[2].clone();
        println!("query={}", query);
        unsafe {
            let c_input = CString::new(query).unwrap();
            let result_ptr = rag_search(c_input.as_ptr());
            if !result_ptr.is_null() {
                let result_cstr = CStr::from_ptr(result_ptr);
                let result_str = result_cstr.to_str().unwrap();
                println!("AI: {}", result_str);
                free_string(result_ptr);
            }
        }
    }

}