// feature 必须放在文件顶部
#![feature(specialization, generic_associated_types)]

use std::ops::Add;

// 本地类型 Int
#[derive(PartialEq)]
struct Int(i32);

// Add trait 是标准库中的, 所以不违反孤儿规则
impl Add<i32> for Int {
    type Output = i32;
    fn add(self, other: i32) -> Self::Output {
        (self.0) + other
    }
}

// Option<T> 会将本地类型变成远程类型, 违反孤儿规则
// error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
// impl Add<i32> for Option<Int> {
//     // TODO
// }

// Box<T> 是标准库中, 但是不违反规则, 该 Box 为特殊处理
impl Add<i32> for Box<Int> {
    type Output = i32;
    fn add(self, other:i32) -> Self::Output {
        (self.0) + other
    }
}

// trait 包含默认实现的特化示例
struct Diver<T> {
    inner: T
}

// 代码有改动
// 见: https://github.com/ZhangHanDong/tao-of-rust-codes/issues/260
trait Swimmer {
    // fn swim(&self) {
    //     println!("swimming")
    // }
    fn swim(&self);
}

impl<T> Swimmer for Diver<T> {
    default fn swim(&self) {
        println!("swimming")
    }
}

impl Swimmer for Diver<&'static str> {
    fn swim(&self) {
        println!("drowning, help!")
    }
}

// 支持 GAT 的 trait 实现示例(泛型关联类型)
// 若给 `std::io::Lines` 实现了 StreamingIterator 迭代器,
// 它就可以复用内存缓存区, 而不需要为每行数据新开辟一份内存, 从此提升性能
trait StreamingIterator {
    // 类型构造器: 类似 `Vec<T>`, 只有在为其制定具体的类型之后才算一个真正的类型
    type Item<'a>;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}


fn main() {
    // 孤儿规则
    assert_eq!(Int(3) + 3, 6);
    assert_eq!(Box::new(Int(3)) + 3, 6);

    let x = Diver::<&'static str> { inner: "Bob" };
    x.swim();
    let y = Diver::<String> { inner: String::from("Alice") };
    y.swim();
}
