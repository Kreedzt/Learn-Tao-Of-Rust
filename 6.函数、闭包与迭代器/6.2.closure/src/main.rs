#![feature(unboxed_closures, fn_traits)]

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
}
