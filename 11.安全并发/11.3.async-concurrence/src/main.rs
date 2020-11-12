#![feature(generators, generator_trait)]
use std::ops::{Generator, GeneratorState};
// 代码变更, 见: https://github.com/ZhangHanDong/tao-of-rust-codes/issues/259
use std::pin::Pin;

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
    let limit = 2;
    let mut gen = up_to_future(limit);

    for i in 0..=limit {
        match Pin::new(&mut gen).resume(()) {
            GeneratorState::Yielded(v) => println!("resume {:?} : Pending", i),
            GeneratorState::Complete(v) => println!("resume {:?} : Ready", i),
        }
    }
}
