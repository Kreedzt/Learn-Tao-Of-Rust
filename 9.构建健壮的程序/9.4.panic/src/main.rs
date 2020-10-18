use std::panic;

fn sum(a: i32, b: i32) -> i32 { a + b }

fn main() {
    // `catch_unwind` 使用示例
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());

    // 代码依旧继续执行
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
    println!("{}", sum(1, 2));


    // 使用 `set_hook` 示例
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());

    panic::set_hook(Box::new(|panic_info| {
        if let Some(location) = panic_info.location() {
            // 自定义错误消息
            println!(
                "panic occurred '{}' at {}",
                location.file(),
                location.line()
            );
        } else {
            println!("can't get location information...");
        }
    }));

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
    println!("{}", sum(1, 2));
}
