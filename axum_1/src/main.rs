use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
    response::{Html, IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::env;
use std::io::{self, Read};
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;

extern "C" {
        fn add(a: i32, b: i32) -> i32;
        fn todo_add(input: *const c_char) -> *mut c_char;
        fn todo_list() -> *mut c_char;
        fn todo_delete(id: i32) -> i32;

        // メモリ解放
        fn free_string(ptr: *mut c_char);
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
    content: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct QueryReq {
    input: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct TodoDeleteReq {
    id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResp {
    ret: String,
    text: String,
} 

async fn get_foo() -> String {
    String::from("foo\n")
}

async fn get_test() -> String {
    unsafe {
        let title = "test1".to_string();
        let c_input = CString::new(title).unwrap();
        unsafe {
            let result_ptr = todo_add(c_input.as_ptr());
            if !result_ptr.is_null() {
                let result_cstr = CStr::from_ptr(result_ptr);
                let result_str = result_cstr.to_str().unwrap();
                println!("C++ returned: '{}'", result_str);
                free_string(result_ptr);
            }
        }         
    }    
    String::from("test\n")
}

async fn get_list_handler() -> Result<Json<SearchResp>, StatusCode> {
    unsafe {
        let result_ptr = todo_list();
        if !result_ptr.is_null() {
            let result_cstr = CStr::from_ptr(result_ptr);
            let result_str = result_cstr.to_str().unwrap();
            //println!("C++ returned: '{}'", result_str);
            let resp_data = SearchResp {
                ret: "OK".to_string(),
                text: result_str.to_string(),
            };
            free_string(result_ptr);
            return Ok(Json(resp_data))                 
        }
    }
    let resp = SearchResp {
        ret: "OK".to_string(),
        text: "".to_string(),
    };
    Ok(Json(resp))     
}


async fn todo_delete_handler(
    Json(payload): Json<TodoDeleteReq>
) -> Result<Json<SearchResp>, StatusCode> {
    unsafe {
        let id: i32 = payload.id;
        let ret = todo_delete(id);
        let resp_data = SearchResp {
            ret: "OK".to_string(),
            text: "".to_string(),
        };
        return Ok(Json(resp_data))                
    }    
    let resp = SearchResp {
        ret: "OK".to_string(),
        text: "".to_string(),
    };
    Ok(Json(resp))                
}

async fn todo_add_handler(
    Json(payload): Json<QueryReq>
) -> Result<Json<SearchResp>, StatusCode> {
    unsafe {
        let title = &payload.input.clone();
        let c_input = CString::new(title.to_string()).unwrap();
        unsafe {
            let result_ptr = todo_add(c_input.as_ptr());
            if !result_ptr.is_null() {
                let result_cstr = CStr::from_ptr(result_ptr);
                let result_str = result_cstr.to_str().unwrap();
                println!("C++ returned: '{}'", result_str);
                let resp_data = SearchResp {
                    ret: "OK".to_string(),
                    text: result_str.to_string(),
                };
                free_string(result_ptr);
                return Ok(Json(resp_data))                
            }
        }         
    }    
    let resp = SearchResp {
        ret: "OK".to_string(),
        text: "".to_string(),
    };
    Ok(Json(resp))                
}
/**
*
* @param
*
* @return
*/
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/foo", get(get_foo))
        .route("/test", get(get_test))
        .route("/api/todo_list", get(get_list_handler))
        .route("/api/todo_add", post(todo_add_handler))
        .route("/api/todo_delete", post(todo_delete_handler))
        ;
    println!("Listening on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
