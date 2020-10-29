use std::thread;

static mut V: i32 = 0;

// 11-1 线程不安全的函数示例
fn unsafe_seq() -> i32 {
    unsafe  {
        // Rust 默认不允许修改静态变量的值
        V += 1;
        V
    }
}

fn main() {
    // 11-6 在多线程环境中使用 `unsafe_req()` 函数
    let child = thread::spawn(move || {
        for _ in 0..10 {
            unsafe_seq();
            unsafe {
                println!("child: {}", V);
            }
        }
    });

    for _ in 0..10 {
        unsafe_seq();
        unsafe {
            println!("main: {}", V);
        }
    }

    child.join().unwrap();
}
