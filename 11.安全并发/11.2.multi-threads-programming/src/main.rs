use std::panic;
use std::thread::{current, spawn, Builder, park, sleep};
use std::time::Duration;
use std::sync::{Arc, Mutex, RwLock};
use std::rc::Rc;
use std::cell::RefCell;
use rand::prelude::*;

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
    // let parked_thread = Builder::new()
    //     .spawn(|| {
    //         println!("Parking thread");
    //         // 阻塞
    //         park();

    //         println!("Thread unparked");
    //     }).unwrap();

    // // 生成毫秒为单位的系统超时时间
    // sleep(Duration::from_millis(10));
    // println!("Unpark the thread");
    // // 使阻塞的线程重启
    // parked_thread.thread().unpark();
    // parked_thread.join().unwrap();


    // 11-17 在线程间传递可变字符串
    // let mut s = "Hello".to_string();
    // for _ in 0..3 {
    //     spawn(move || {
    //         // 违反了 Rust 所有权机制
    //         // |               ^^^^^^^ value moved into closure here, in previous iteration of loop
    //         s.push_str(" Rust!");
    //     });
    // }


    // 11-18 尝试使用 `Rc` 共享所有权
    // let mut s = Rc::new("Hello".to_string());
    // for _ in 0..3 {
    //     let mut s_clone = s.clone();
    //     spawn(move || {
    //         // | |         `Rc<String>` cannot be sent between threads safely
    //         s_clone.push_str(" hello");
    //     });
    // }


    // 11-19 使用 `Arc` 共享所有权
    // let s = Arc::new("Hello".to_string());
    // for _ in 0..3 {
    //     let s_clone = s.clone();
    //     spawn(move || {
    //         s_clone.push_str(" world!");
    //         // |             ^^^^^^^ cannot borrow as mutable
    //     });
    // }


    // 11-20 使用 `RefCell` 支持内部可变性.
    // let s = Arc::new(RefCell::new("Hello".to_string()));
    // for _ in 0..3 {
    //     let s_clone = s.clone();
    //     spawn(move || {
    //         // error[E0277]: `RefCell<Strisng>` cannot be shared between threads safely
    //         let s_clone = s_clone.borrow_mut();
    //         s_clone.push_str(" world!");
    //     });
    // }


    // 11-21 使用 `Mutex` 在多线程环境中共享可变变量
    // let s = Arc::new(Mutex::new("Hello".to_string()));
    // let mut v = vec![];
    
    // for _ in 0..3 {
    //     let s_clone = s.clone();
    //     let child = spawn(move || {
    //         let mut s_clone = s_clone.lock().unwrap();
    //         // 存在竞态条件, 该操作可能乱序执行.
    //         s_clone.push_str(" world!");
    //     });
    //     v.push(child);
    // }

    // for child in v {
    //     child.join().unwrap();
    // }


    // 11-22 "中毒" 示例
    // let mutex = Arc::new(Mutex::new(1));
    // let c_mutex = mutex.clone();
    // let _ = spawn(move || {
    //     let mut data = c_mutex.lock().unwrap();
    //     // `data` 为 `MutexGuard<T>` 类型, 该类型实现了 `Deref` 和 `DerefMut`.
    //     *data = 2;
    //     panic!("oh no");
    // }).join();

    // assert_eq!(mutex.is_poisoned(), true);

    // match mutex.lock() {
    //     Ok(_) => unreachable!(),
    //     // 因为在子线程内部发生了恐慌, 所以这里只会返回 `Err`
    //     // 该 `Err` 是 `PoisonError<T>` 类型, 提供了 `get_ref()` 或 `get_mut()` 方法
    //     Err(p_err) => {
    //         let data = p_err.get_ref();
    //         println!("recovered: {}", data);
    //     }
    // };


    // 11-24 完善抛硬币 `main` 函数
    let total_flips = Arc::new(Mutex::new(0));
    let completed = Arc::new(Mutex::new(0));
    let runs = 8;
    let target_flips = 10;

    for _ in 0..runs {
        let total_flips = total_flips.clone();
        let completed = completed.clone();

        spawn(move || {
            flip_simulate(target_flips, total_flips);
            let mut completed = completed.lock().unwrap();
            *completed += 1;
        });
    }

    // loop {
    //     let completed = completed.lock().unwrap();
    //     if *completed == runs {
    //         let total_flips = total_flips.lock().unwrap();
    //         println!("Final average: {}", *total_flips / *completed);
    //         break;
    //     }
    // }

    
    // 11-26 改为死锁代码
    // 始终持有互斥锁, 会导致所有子线程阻塞
    // 间接导致无法更新 `completed` 的值
    // let completed = completed.lock().unwrap();
    // while *completed < runs { }
    // let total_flips = total_flips.lock().unwrap();
    // println!("Final average: {}", *total_flips / *completed);


    // 11-27 读写锁示例
    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5)
    }
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }
}


// 11-23 模拟掷硬币函数
fn flip_simulate(target_flips: u64, total_flips: Arc<Mutex<u64>>) {
    // 正面次数
    let mut countinue_positive = 0;
    // 掷硬币次数
    let mut iter_counts = 0;

    while countinue_positive < target_flips {
        iter_counts += 1;
        // 获取随机的 `bool` 类型
        let pro_or_con = rand::random();

        // 推断出随机函数值的类型
        if pro_or_con {
            countinue_positive += 1;
        } else {
            countinue_positive = 0;
        }
    }

    println!("iter_counts: {}", iter_counts);
    let mut total_flips = total_flips.lock().unwrap();
    *total_flips += iter_counts;
}
