use std::cell::RefCell;
use std::panic;
use std::thread::{current, spawn, Builder, park, sleep};
use std::time::Duration;

fn main() {
    // 11-7 创建线程
    // let mut v = vec![];

    // for id in 0..5 {
    //     // 接收闭包作为参数
    //     let child = spawn(move || {
    //         // 默认按引用捕获, 此处需要所有权转移
    //         println!("in child: {}", id);
    //     });

    //     v.push(child);
    // }

    // println!("in main: join before");

    // for child in v {
    //     // `join()` 可以让 `main` 主线程等待这些子线程都指向完毕
    //     child.join();
    // }

    // 该行永远输出在结尾
    // println!("in main: join after");


    // 11-9 使用 `thread::Builder` 来定制线程
    // let mut v = vec![];
    // for id in 0..5 {
    //     let thread_name = format!("child-{}", id);
    //     let size: usize = 3 * 1024;
    //     let builder = Builder::new().name(thread_name).stack_size(size);

    //     let child = builder
    //         .spawn(move || {
    //             println!("in child: {}", id);
    //             if id == 3 {
    //                 panic::catch_unwind(|| {
    //                     panic!("oh no!");
    //                 });
    //                 // 若不给定线程明显, 默认显示 ~unknow~
    //                 println!("in {} do sm", current().name().unwrap());
    //             }
    //         })
    //         .unwrap();

    //     v.push(child);
    // }

    // for child in v {
    //     child.join().unwrap();
    // }


    // 11-11 线程本地存储示例
    // thread_local! {
    //     static FOO: RefCell<u32> = RefCell::new(1);
    // }

    // FOO.with(|f| {
    //     assert_eq!(*f.borrow(), 1);
    //     // 修改线程本地存储内部的值
    //     *f.borrow_mut() = 2;
    // });

    // spawn(|| {
    //     FOO.with(|f| {
    //         assert_eq!(*f.borrow(), 1);
    //         // 修改线程本地存储内部的值
    //         *f.borrow_mut() = 3;
    //     });
    // });

    // FOO.with(|f| {
    //     // 并没有因为子线程的修改而发生变化
    //     assert_eq!(*f.borrow(), 2);
    // });


    // 11-12 `park` 和 `unpark` 函数使用示例
    let parked_thread = Builder::new()
        .spawn(|| {
            println!("Parking thread");
            // 阻塞
            park();

            println!("Thread unparked");
        }).unwrap();

    // 生成毫秒为单位的系统超时时间
    sleep(Duration::from_millis(10));
    println!("Unpark the thread");
    // 使阻塞的线程重启
    parked_thread.thread().unpark();
    parked_thread.join().unwrap();
}
