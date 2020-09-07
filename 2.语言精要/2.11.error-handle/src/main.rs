use std::fs::File;

// 参考源码实现
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

// 仅限 Rust 2018
// fn main() {
//     let x:Result<i32, &str> = Ok(-3);
//     assert_eq!(x.is_ok(), true);
//     let x: Result<i32, &str> = Err("Some error message");
//     assert_eq!(x.is_ok(), false);
// }

fn main() -> Result<(), std::io::Error> {
    // ?操作符: 语法糖, 会自动在出错的情况下提前返回错误: std::io::Error
    // 并在退出程序时打印相关的信息
    let f = File::open("bar.txt")?;
    Ok(())
}
