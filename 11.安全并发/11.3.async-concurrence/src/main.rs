#![feature(generators, generator_trait)]
use std::ops::{Generator, GeneratorState};
// 代码变更, 见: https://github.com/ZhangHanDong/tao-of-rust-codes/issues/259
use std::future;
use std::marker::{Unpin};
use std::pin::{Pin};
use std::ptr::NonNull;

// 11-66 将生成器用作迭代器
pub fn up_to() -> impl Generator<Yield = u64, Return = ()> {
    || {
        let mut x = 0;
        loop {
            x += 1;
            yield x;
        }
        return ();
    }
}


// 11-67 用生成器模拟 Future
fn up_to_future(limit: u64) -> impl Generator<Yield = (), Return = Result<u64, ()>> {
    move || {
        for x in 0..limit {
            yield ();
        }
        return Ok(limit);
    }
}

// TODO: 无法编译
// struct Unmoveable {
//     data: String,
//     slice: NonNull<String>,
//     _pin: Pinned,
// }

// impl Unpin for Unmoveable {}

// impl Unmoveable {
//     fn new(data: String) -> Pin<Box<Self>> {
//         let res = Unmoveable {
//             data,
//             slice: NonNull::dangling(),
//             _pin: Pinned,
//         };

//         let mut boxed = Box::pinned(res);
//         let slice = NonNull::from(&boxed.data);

//         unsafe {
//             let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
//             Pin::get_unchecked_mut(mut_ref).slice = slice;
//         }
//         boxed
//     }
// }

fn main() {
    // 11-62 Rust 生成器的用法
    // let mut gen = || {
    //     yield 1;
    //     yield 2;
    //     yield 3;
    //     return 4;
    // };

    // for _ in 0..4 {
    //     // 调用 resume() 方法
    //     let c = Pin::new(&mut gen).resume(());
    //     println!("{:?}", c);
    // }


    // 11-66 将生成器用作迭代器
    // let mut gen = up_to();
    // for _ in 0..10 {
    //     match Pin::new(&mut gen).resume(()) {
    //         GeneratorState::Yielded(i) => println!("{:?}", i),
    //         _ => println!("Completed"),
    //     }
    // }

    // 11-67 用生成器模拟 Future
    // let limit = 2;
    // let mut gen = up_to_future(limit);

    // for i in 0..=limit {
    //     match Pin::new(&mut gen).resume(()) {
    //         GeneratorState::Yielded(v) => println!("resume {:?} : Pending", i),
    //         GeneratorState::Complete(v) => println!("resume {:?} : Ready", i),
    //     }
    // }

    // 11-74 修改代码清单 11-62 中的生成器实例
    // TODO: 编译通过?, 理应报错
    // let mut gen = || {
    //     let x = 1u64;
    //     let ref_x = &x;
    //     yield 1;
    //     yield 2;
    //     yield 3;
    //     return 4;
    // };


    // 11-76 `Pin<T>` 用法示例
    // TODO: 无法编译
    // let unmoved = Unmoveable::new("hello".to_string());
    // let mut still_unmoved = unmoved;
    // assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));
    // let mut new_unmoved = Unmoveable::new("world".to_string());
    // std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
}
