// 外部関数を宣言
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
        let result = add(5, 3);
        println!("5 + 3 = {}", result);
    }
}