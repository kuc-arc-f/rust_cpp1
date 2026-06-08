use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;
use std::env;

extern "C" {
    fn add(a: i32, b: i32) -> i32;
    fn todo_add(input: *const c_char) -> *mut c_char;
    fn todo_list() -> i32;
    fn todo_delete(id: i32) -> i32;

    // 文字列受信（Rust → C++）
    fn receive_string(msg: *const c_char);
    
    // 文字列送信（C++ → Rust）
    fn send_string() -> *mut c_char;
    
    // 文字列処理（受信→加工→返却）
    fn process_string(input: *const c_char) -> *mut c_char;
    
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

    if cmd == "add" {
        if args.len() < 3 {
            println!("error: no argument");
            return;
        }           
        let title = args[2].clone();
        println!("title={}", title);
        unsafe {
            let c_input = CString::new(title).unwrap();
            unsafe {
                let result_ptr = todo_add(c_input.as_ptr());
                if !result_ptr.is_null() {
                    let result_cstr = CStr::from_ptr(result_ptr);
                    let result_str = result_cstr.to_str().unwrap();
                    //println!("Rust sent: '{}'", title.clone());
                    println!("C++ returned: '{}'", result_str);
                    free_string(result_ptr);
                }
            }         
        }
    }
    if cmd == "list" {
        unsafe {
            let ret = todo_list();
        }
    }
    if cmd == "delete" {
        if args.len() < 3 {
            println!("error: no argument");
            return;
        }           
        let id_str = args[2].clone();
        println!("id_str={}", id_str); 
        let num2 = id_str.parse::<i32>().unwrap();        
        println!("{}", num2);               
        unsafe {
            let ret = todo_delete(num2);
        }
    }

}