#![feature(unboxed_closures, fn_traits)]
use std::fmt::Debug;
// error[E0635]: unknown feature `fnbox`
// #![feature(unboxed_closures, fn_traits, fnbox)]

// 返回闭包
// 放入 `Box<T>` 中是因为闭包的大小在编译期是未知的.
// fn counter(i: i32) -> Box<Fn(i32) -> i32> {
//     Box::new(move |n: i32| n + i)
// }

// Rust 2018 中也可以写成 `impl Fn(i32) -> i32`
fn counter(i: i32) -> impl Fn(i32) -> i32 {
    // `i` 为自由变量
    // `i` 为复制语义类型, 所以它肯定会按引用被捕获. 此引用会妨碍闭包作为函数返回值, 编译器会报错
    // 所以使用 `move` 关键字来把自由变量 `i` 的所有权转移到闭包中.
    // 因为变量 `i` 是复制语义, 所以这里只会进行按位复制
    Box::new(move |n: i32| n + i)
}

fn val() -> i32 { 5 }

// 模拟编译器对闭包的实现
struct Closure {
    // 代表从环境中捕获的自由变量
    env_var: u32,
}

impl FnOnce<()> for Closure {
    type Output = u32;
    extern "rust-call" fn call_once(self, args: ()) -> u32 {
        println!("call it FnOnce()");
        self.env_var + 2
    }
}

impl FnMut<()> for Closure {
    extern "rust-call" fn call_mut(&mut self, args: ()) -> u32 {
        println!("call it FnMut()");
        self.env_var + 2
    }
}

impl Fn<()> for Closure {
    extern "rust-call" fn call(&self, args: ()) -> u32 {
        println!("call it Fn()");
        self.env_var + 2
    }
}

fn call_it<F: Fn() -> u32>(f: &F) -> u32 { f() }

fn call_it_mut<F: FnMut() -> u32>(f: &mut F) -> u32 { f() }

fn call_it_once<F: FnOnce() -> u32>(f: F) -> u32 { f() }

// 闭包为翻译为匿名结构体和 trait 的情况
struct Closure2<'a> {
    env_var: &'a u32,
}

impl<'a> FnOnce<()> for Closure2<'a> {
    type Output = ();
    extern "rust-call" fn call_once(self, args: ()) -> () {
        println!("{:?}", self.env_var);
    }
}

impl<'a> FnMut<()> for Closure2<'a> {
    extern "rust-call" fn call_mut(&mut self, args: ()) -> () {
        println!("{:?}", self.env_var);
    }
}

impl<'a> Fn<()> for Closure2<'a> {
    extern "rust-call" fn call(&self, args: ()) -> () {
        println!("{:?}", self.env_var);
    }
}

// 使用 `FnOnce()` 闭包作为参数
// 在函数体内执行闭包, 用于判断自身的所有权是否转移
fn call<F: FnOnce()>(f: F) { f() }

fn boxed_closure(c: &mut Vec<Box<Fn()>>) {
    let s = "second";
    c.push(Box::new(|| println!("first")));
    // 以不可变方式捕获了环境变量 `s`,
    // 但这里需要将闭包装箱稍后在迭代器中使用
    // 所以这里必须使用 `move` 关键字将 `s` 的所有权转移到闭包中,
    // 因为变量 `s` 是复制语义类型, 所以该闭包捕获的是原始变量 `s` 的副本
    c.push(Box::new(move || println!("{}", s)));
    c.push(Box::new(|| println!("third")));
}

// `Fn` 并不受孤儿规则限制, 可有可无
// use std::ops::Fn;

// 以 trait 限定的方式实现 any 方法
// 自定义的 Any 不同于标准库的 Any
// 该函数的泛型 `F` 的 trait 限定为 `Fn(u32) -> bool`
// 有别于一般的泛型限定 `<F: Fn<u32, bool>>`
trait Any {
    fn any<F>(&self, f: F) -> bool
    where
        Self: Sized,
        F: Fn(u32) -> bool;
}

impl Any for Vec<u32> {
    fn any<F>(&self, f: F) -> bool
    where
        // Sized 限定该方法不能被动态调用, 这是一种优化策略
        Self: Sized,
        F: Fn(u32) -> bool,
    {
        // 迭代传递的闭包, 依次调用
        for &x in self {
            if f(x) {
                return true;
            }
        }

        false
    }
}

// 函数指针也可以作为闭包参数
fn call_ptr<F>(closure: F) -> i32
where
    F: Fn(i32) -> i32,
{
    closure(1)
}

fn counter_ptr(i: i32) -> i32 { i + 1 }

// 将闭包作为 trait 对象进行动态分发
trait AnyDyn {
    fn any_dyn(&self, f: &(Fn(u32) -> bool)) -> bool;
}

impl AnyDyn for Vec<u32> {
    fn any_dyn(&self, f: &(Fn(u32) -> bool)) -> bool {
        for &x in self.iter() {
            if f(x) {
                return true;
            }
        }

        false
    }
}

// 将闭包作为函数返回值
// `Fn` 可以多次调用
fn square() -> Box<Fn(i32) -> i32> { Box::new(|i| i * i) }

// 指定返回闭包为 `FnOnce`
fn square_once() -> Box<FnOnce(i32) -> i32> { Box::new(|i| i * i) }

// impl Trait 示例
// 在 impl 关键字后面加上了闭包 trait, 这样就可以直接返回一个 `FnOnce trait`
fn square_impl() -> impl FnOnce(i32) -> i32 { |i| i * i }

// 泛型 trait 作为 trait 对象时的生命周期参数
trait DoSomething<T> {
    fn do_sth(&self, value: T);
}

// 为 usize 类型实现该 trait
impl<'a, T: Debug> DoSomething<T> for &'a usize {
    fn do_sth(&self, value: T) {
        println!("{:?}", value);
    }
}

// usize 是从外部引入的, 跟 foo 函数没有直接关系
// fn foo<'a>(b: Box<DoSomething<&'a usize>>) {
//     let s: usize = 10;
//     // s 在调用结束被析构
//     // &s 会成为悬垂指针
//     b.do_sth(&s)
// }

// 使用高阶生命周期: `for<>` 语法
fn bar<'a>(b: Box<for<'f> DoSomething<&'f usize>>) {
    let s: usize = 10;
    // s 在调用结束被析构
    // &s 会成为悬垂指针
    b.do_sth(&s)
}

// 闭包参数和返回值都是引用类型的情况
struct Pick<F> {
    data: (u32, u32),
    func: F,
}

// 编译器自动补齐了生命周期参数
// impl<F> Pick<F>
// where
//     F: Fn(&(u32, u32)) -> &u32,
// {
//     fn call(&self) -> &u32 { (self.func)(&self.data) }
// }

// 实际生命周期
impl<F> Pick<F>
where
    F: for<'f> Fn(&'f (u32, u32)) -> &'f u32,
{
    fn call(&self) -> &u32 { (self.func)(&self.data) }
}

fn max(data: &(u32, u32)) -> &u32 {
    if data.0 > data.1 {
        &data.0
    } else {
        &data.1
    }
}

fn main() {
    //
    let f = counter(3);
    assert_eq!(4, f(1));

    // 闭包的参数可以为任意类型
    // a: 函数指针, (b, c): 元组, 会通过函数指针类型的信息自动推断元组内为 i32 类型
    let add = |a: fn() -> i32, (b, c)| (a)() + b + c;
    let r = add(val, (2, 3));
    assert_eq!(r, 10);

    // 两个相同定义的闭包却不属于同一种类型
    // Rust 2018 已修复
    let c1 = || {};
    let c2 = || {};
    let v = [c1, c2];

    // 查看闭包的类型
    // let c1: () = || println!("i'm a closure");
    // |             --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found closure

    // 模拟编译器对闭包的实现
    let env_var = 1;
    let mut c = Closure { env_var: env_var };
    // 实例调用
    // 实际由 ABI 实现("rust-call")
    c();
    // 必须显式指定一个单元值作为参数
    c.call(());
    // 必须显式指定一个单元值作为参数
    c.call_mut(());
    // 必须显式指定一个单元值作为参数
    // `call_once` 调用之后, 之前的实例所有权被转移, 无法再次被使用.
    c.call_once(());
    let mut c = Closure { env_var: env_var };
    {
        assert_eq!(3, call_it(&c));
    }
    {
        assert_eq!(3, call_it_mut(&mut c));
    }
    {
        assert_eq!(3, call_it_once(c));
    }

    // 与上者等价的闭包示例
    let env_var = 1;
    let c = || env_var + 2;
    assert_eq!(3, c());

    // 显式指定闭包类型
    let env_var = 1;
    // 该类型为 trait 对象, 此处必须使用 trait 对象
    let c: Box<Fn() -> i32> = Box::new(|| env_var + 2);
    assert_eq!(3, c());

    // 复制语义类型自动实现 `Fn`
    // 绑定为字符串字面量, 为复制语义类型
    let s = "hello";
    // 闭包会按照不可变引用类型来捕获 `s`
    // 该闭包默认自动实现了 `Fn` 这个 trait, 并且该闭包以不可变借用捕获环境中的自由变量
    let c = || println!("{:?}", s);
    c();
    // 闭包 c 可以多次调用, 说明编译器自动为闭包表达式实现的结构体实例并未失去所有权.
    c();
    // 对 s 进行一次不可变借用
    println!("{:?}", s);

    // 闭包被翻译为匿名结构体和 trait 的情况
    // 闭包被翻译为结构体 `Closure<'a>`, 因为环境变量是按不可变
    let env_var = 42;
    let mut c = Closure2 { env_var: &env_var };
    c();
    c.call_mut(());
    c.call_once(());

    // 实现了 `Fn` 的闭包也可以显式调用 `call_mut` 和 `call_once`
    let s = "hello";
    let mut c = || println!("{:?}", s);
    c();
    c();
    // 依赖 `#[feature(fn_traits)]` 特性(如果不是默认的闭包调用, 并不需要此特性)
    // 实现了 `Fn` 的闭包也可以显式调用 `call_mut` 和 `call_once` 方法
    c.call_mut(());
    c.call_once(());
    c;
    println!("{:?}", s);

    // 移动语义类型自动实现 `FnOnce`
    let s = "hello".to_string();
    // 编译器翻译的闭包结构体中记录捕获变量的成员不是引用类型, 并且只实现 `FnOnce`
    // error[E0525]: expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`
    // error[e0525]: expected a closure that implements the `fnmut` trait, but this closure only implements `fnonce`
    let c = || s;
    c();
    // error[e0382]: use of moved value: `c`
    // c();
    // c.call(());
    // c.call_mut(());

    // 环境变量为复制语义类型时使用 `move` 关键字
    let s = "hello";
    // 虽然 `move` 关键字强制执行, 但闭包捕获的 `s` 执行的对象是复制语义后获取的新变量.
    // 原始的 `s` 并未失去所有权.
    // 所以肯定是 `&self` 和 `&mut self` 中的一种
    // 又因为闭包 c 是不可变的, 所以只存在 `&self`;
    // 可变借用需要使用 `mut` 关键字将 c 本身修改为可变
    let c = move || println!("{:?}", s);

    c();
    c();
    println!("{:?}", s);

    // 环境变量为移动语义的情况
    // 移动语义类型 `String`
    let s = "hello".to_string();
    // 使用 move 后无法再次使用
    let c = move || println!("{:?}", s);
    c();
    c();
    // error[E0382]: borrow of moved value: `s`
    // println!("{:?}", s);

    // move 关键字是否影响闭包本身
    let mut x = 0;
    let incr_x = || x += 1;
    call(incr_x);
    // error[E0382]: use of moved value: `incr_x`
    // call(incr_x);
    // 使用 move
    let mut x = 0;
    let incr_x = move || x += 1;
    call(incr_x);
    call(incr_x);
    println!("x: {}", x);
    // 对移动语义类型使用 `move`
    let mut x = vec![];
    let expand_x = move || x.push(42);
    call(expand_x);
    // error[E0382]: use of moved value: `expand_x`
    // call(expand_x);

    // 修改环境变量的闭包来自动实现 `FnMut`
    // 使用 mut 关键字修改了其可变性, 变成了可变绑定
    let mut s = "rust".to_string();
    {
        // 通过闭包实现自我修改, 所以需要声明 mut
        // 如果想修改环境变量, 必须实现 `FnMut`
        // 由编译器生成的闭包结构体实例在调用 `FnMut` 方法时, 需要 `&mut self`
        let mut c = || s += " rust";
        c();
        // 改动源: https://github.com/ZhangHanDong/tao-of-rust-codes/issues/103
        // 这行本应该出错, 但因为 NLL 的支持, 没有出错.
        c();
        println!("{:?}", s);
    } // 归还了所有权
    println!("{:?}", s);

    // 实现了 `FnMut` 的闭包的情况
    let mut s = "rust".to_string();
    {
        // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
        let mut c = || s += " rust";
        c();
        // 闭包只实现了 `FnMut`, 没有实现 `Fn`
        // c.call(());
        c.call_once(());
        println!("{:?}", s);
    }
    println!("{:?}", s);

    // 没有捕获任何环境变量的闭包
    // 没有捕获环境变量, 没有使用 `mut` 关键字, 然而可以多次调用
    // 足以证明编译器为其自动实现的结构体实例并未失去所有权, 只可能是 `&self`
    // 所以, 闭包一定实现了 `Fn`
    let c = || println!("hhh");
    c();
    c();

    // 把闭包当作 trait 对象
    // `Box<Fn()>` 是一个 trait 对象
    // 把闭包放到 `Box<T>` 中就可以构建一个闭包的 trait 对象
    // trait 对象是动态分发的
    let mut c: Vec<Box<Fn()>> = vec![];
    boxed_closure(&mut c);
    for f in c {
        f();
    }

    // 以 trait 限定的方式实现 any 方法
    let v = vec![1, 2, 3];
    let b = v.any(|x| x == 3);
    println!("{:?}", b);

    // 函数指针也可以作为闭包参数
    // 函数指针也实现了 `Fn`
    let result = call_ptr(counter_ptr);
    assert_eq!(2, result);

    // 将闭包作为 trait 对象进行动态分发
    let v = vec![1, 2, 3];
    let b = v.any_dyn(&|x| x == 3);
    println!("{:?}", b);

    // 测试 `'static` 约束
    let s = "hello";
    // let c: Box<Fn() + 'static> = Box::new(move || { s; });
    // error[E0597]: `s` does not live long enough
    // let c: Box<Fn() + 'static> = Box::new(|| { s; });

    // 将闭包作为函数返回值
    let square_rt = square();
    assert_eq!(4, square_rt(2));
    assert_eq!(9, square_rt(3));

    // 指定闭包返回为 `FnOnce`
    // 内容有变动: https://github.com/ZhangHanDong/tao-of-rust-codes/issues/249
    let square_once_rt = square_once();
    assert_eq!(4, square_once_rt(2));

    // impl Trait 示例
    let square_impl_rt = square_impl();
    assert_eq!(4, square_impl_rt(2));

    // 泛型 trait 作为 trait 对象时的生命周期参数
    let x = Box::new(&2usize);
    // foo(x);

    bar(x);

    // 闭包参数和返回值都是引用类型的情况
    let elm = Pick {
        data: (3, 1),
        func: max,
    };
    println!("{}", elm.call());
}
