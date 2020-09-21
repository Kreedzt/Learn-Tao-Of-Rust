// use std::ops::Add as OAdd;
use std::ops::Add;
use std::fmt::Display;
use std::fmt::Debug;
use std::thread;
use std::rc::Rc;

// RHS 和 Output 分别代表加法操作符右侧的类型和返回值的类型
trait MAdd<RHS, Output> {
    fn my_add(self, rhs: RHS) -> Output;
}

impl MAdd<i32, i32> for i32 {
    fn my_add(self, rhs: i32) -> i32 {
        self + rhs
    }
}

impl MAdd<u32, i32> for u32 {
    fn my_add(self, rhs: u32) -> i32 {
        (self + rhs) as i32
    }
}

// 标准库 Add trait 的定义
// pub trait Add<RHS = Self> {
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }

// 尝试重载整数的加法
// error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
// impl Add<u64> for u32 {
//     type Output = u64;
//     fn add(self, other: u64) -> Self::Output {
//         (self as u64) + other
//     }
// }

// 当前 crate 定义
trait LAdd<RHS=Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

impl LAdd<u64> for u32 {
    type Output = u64;
    // 重载, 若要使用, 此时不能use std::ops::Add
    fn add(self, other: u64) -> Self::Output {
        (self as u64) + other
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;
    // 返回值可以写 Point, Self, 和 Self::Output
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

// 继承 trait 示例 - 分页
trait Page {
    // 默认实现
    fn set_page(&self, p: i32) {
        println!("Page Default: 1");
    }
}

trait PerPage {
    // 默认实现
    fn set_perpage(&self, num: i32) {
        println!("Per Page Default: 10");
    }
}

struct MyPaginate {
    page: i32
}

impl Page for MyPaginate {}
impl PerPage for MyPaginate {}

// 修正泛型求和函数
// 使用泛型约束
fn sum<T: Add<T, Output=T>>(a: T, b: T) -> T {
    a + b
}

// 使用 trait 继承扩展功能
trait Paginate: Page + PerPage {
    fn set_skip_page(&self, num: i32) {
        println!("Skip Page: {:?}", num);
    }
}

// 为有 Page 和 PerPage 行为的类型实现 Paginate
impl <T: Page + PerPage>Paginate for T {}

// trait 限定和 trait 对象的用法比较
#[derive(Debug)]
struct Foo;
trait Bar {
    fn baz(&self);
}

impl Bar for Foo {
    fn baz(&self) {
        println!("{:?}", self);
    }
}

// 带 trait 泛型限定的泛型函数 -- 静态分发
fn static_dispatch<T>(t: &T) where T: Bar {
    t.baz();
}

// 使用 trait 对象 -- 动态分发
fn dynamic_dispatch(t: &dyn Bar) {
    t.baz();
}

// 标记为 Sized 的 trait
// 无法作为 trait 对象
trait NFoo: Sized {
    fn Some_method(&self);
}

// error[E0038]: the trait `NFoo` cannot be made into an object
// fn dynamic_dispatch_NFoo(t: &NFoo) {
//     t.Some_method();
// }

// 标准的对象安全的的 trait
// 该 trait 不受 Sized 限定
// 方法都是没有额外 Self 类型参数的非泛型成员方法
trait SafeBar {
    fn bax(self, x: u32);
    fn bay(&self);
    fn baz(&mut self);
}

// 对象不安全的 trait
trait UnsafeFoo {
    // 泛型参数,不安全
    fn bad<T>(&self, x: T);
    fn new() -> Self;
}

// 对象安全的 trait, 将不安全的方法拆分出去
// trait SFoo {
//     fn bad<T>(&self, x: T);
// }

// trait SFoo: SafeBar {
//     fn new() -> Self;
// }

// 对象安全的 trait
trait SafeFoo {
    // 使用 Sized 限定 Self
    fn new() -> Self where Self: Sized;
}

// impl Trait
pub trait Fly {
    fn fly(&self) -> bool;
}

#[derive(Debug)]
struct Duck;

#[derive(Debug)]
struct Pig;

impl Fly for Duck {
    fn fly(&self) -> bool {
        true
    }
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        false
    }
}

fn fly_static(s: impl Fly + Debug) -> bool {
    s.fly()
}

fn can_fly(s: impl Fly + Debug) -> impl Fly {
    if s.fly() {
        println!("{:?} can fly", s);
    } else {
        println!("{:?} can't fly", s);
    }

    s
}

// a, b 同时使用了 impl Trait 语法, 编译器认为 2 个类型, 所以报错
// fn m_sum<T>(a: impl Add<Output=T>, b: impl Add<Output=T>) -> T {
//     // error[E0308]: mismatched types
//     a + b
// }

// 仅仅是 识别 为两个不同的类型, 使用依旧可以
fn m_sum<T>(a: impl Add<Output=T> + Display, b: impl Add<Output=T> + Display) {
    println!("{}", a);
    println!("{}", b);
}

// dyn Trait 语法
// 返回的 trait 对象
// 'static 是一种生命周期参数, 限定了 impl Fly + Debug 抽象类型不可能是引用类型, 引用类型可能会引发内存不安全
fn dyn_can_fly(s: impl Fly + Debug + 'static) -> Box<dyn Fly> {
    if s.fly() {
        println!("{:?} can fly", s);
    } else {
        println!("{:?} can't fly", s);
    }

    Box::new(s)
}

// 实现 Copy trait.
// struct MyStruct;
// impl Copy for MyStruct {}
// 实现 Copy trait 必须实现 Clone trait
// impl Clone for MyStruct {
//     fn clone(&self) -> MyStruct {
//         *self
//     }
// }

// 通过 derive
#[derive(Copy, Clone)]
struct MyStruct;

// 检测类型是否实现了 Copy trait
fn test_copy<T: Copy>(i: T) {
    println!("hhh");
}

fn main() {
    let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
    let x: i32 = a.my_add(b);
    let y: i32 = c.my_add(d);

    assert_eq!(x, 3i32);
    assert_eq!(y, 7i32);

    // String 类型字符串的加法运算
    let a = "hello";
    let b = " world";

    // 并没有为 &str 自身实现加法
    // let c = a + b;
    // 只有 String 类型才实现了加法
    let c = a.to_string() + b;
    println!("{:?}", c);

    // 尝试重载整数的加法
    // let a = 1u32;
    // let b = 2u64;
    // // assert_eq!(a + b, 3);
    // assert_eq!(a.add(b), 3);

    // 为新类型实现 Add
    println!("{:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });

    // 分页为例 - 继承 trait 示例
    let my_paginate = MyPaginate { page: 1 };
    my_paginate.set_page(2);
    my_paginate.set_perpage(100);

    // 调用扩展的方法
    my_paginate.set_skip_page(12);

    // trait 限定和 trait 对象用法比较
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);

    // 调用 impl Trait 语法
    let pig = Pig;
    assert_eq!(fly_static(pig), false);

    let duck = Duck;
    assert_eq!(fly_static(duck), true);

    let pig = Pig;
    // 因为 参数和返回值都使用了 impl Fly, 所以此处不必使用 turbofish 操作符
    let pig = can_fly(pig);

    let duck = Duck;
    // 因为 参数和返回值都使用了 impl Fly, 所以此处不必使用 turbofish 操作符
    let duck = can_fly(duck);

    m_sum(1, 2);

    let duck = Duck;
    // 调用含有 dyn Trait 定义的函数签名的函数
    let res = dyn_can_fly(duck);

    // 检测类型是否实现了 Copy trait
    let a = "String".to_string();
    // String 并没有实现 Copy trait
    // error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
    // test_copy(a);

    // 尝试在多线程之间共享可变变量
    let mut x = vec![1, 2, 3, 4];
    // 若编译通过, 父子线程都可以访问共享的可变变量, 就有可能出现数据竞争的问题
    // error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
    // thread::spawn(|| {
    //     // 闭包中的 x 实际为借用, 无法确定本地变量 x 可以比闭包中的 x 存活的更久.
    //     // 如果 x 被释放了, 闭包中的 x 借用就成了悬垂指针, 造成内存不安全.
    //     x.push(1);
    // });

    // x.push(2);

    // 使用 move 转移了所有权
    // 向量 x 中的数据都为原生数据类型, 默认都实现了 `Send` 和 `Sync` 标签 trait
    // 所以跨线程传递和访问都很安全
    // 在 x 被转移后, 就不允许父线程对其 x 进行修改.
    // thread::spawn(move || x.push(1));

    let x = Rc::new(vec![1, 2, 3, 4]);

    // 在多线程之间传递没有实现 `Send` 和 `Sync` 的类型
    // error[E0277]: `std::rc::Rc<std::vec::Vec<i32>>` cannot be sent between threads safely
    // thread::spawn(move || {
    //     x[1];
    // });
}
