// 13-51 在 Rust 中调用 C 标准库函数
// extern "C" {
//     fn isalnum(input: i32) -> i32;
// }

// 默认即为 C-ABI
extern {
    fn isalnum(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Is 3 a number ? the answer is : {}", isalnum(3));
        // Rust 的检查依旧生效
        // println!("Is 'a' a number ? ", isalnum('a'));
    }
}
