use std::ops::Mul;

// 函数定义示例
// 函数定义形式
fn func_name(arg1: u32, arg2: String) -> Vec<u32> {
    // 函数体
    vec![]
}

// 利用 Raw identifier 将语言关键字用作函数名(Rust 2018 版本)
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

// 按值传递的参数使用 `mut` 关键字
fn modify(mut v: Vec<u32>) -> Vec<u32> {
    v.push(42);
    v
}

// error[E0428]: the name `modify` is defined multiple times
// fn modify() {

// }

// 按引用传递参数时的 `mut` 用法
fn modify_ref(v: &mut [u32]) {
    v.reverse()
}

// 作用域内的函数会屏蔽作用域外的同名函数
fn f() {
    print!("1");
}

// 函数参数支持模式匹配
#[derive(Debug)]
struct S {
    i: i32,
}
// ref 关键字修饰: 意味着要使用模式匹配来获取参数的不可变引用.
// 相对应: `ref mut` 用来匹配可变引用
fn f_fn(ref _s: S) {
    println!("{:p}", _s);
}

// 使用通配符忽略参数
fn foo(_: i32) {
    // ...
}

// 函数参数利用模式匹配来解构元组
fn swap((x, y): (&str, i32)) -> (i32, &str) {
    (y, x)
}

// 使用元组类型让函数返回多个值
fn addsub(x: isize, y: isize) -> (isize, isize) {
    (x + y, x - y)
}

// 使用 `return` 提前返回示例
// 函数使用欧几里得算法(辗转相除法) 求两数中的最大公约数
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

// 实现泛型函数示例
// Mul trait 有关联类型, 所以这里需要显式指定为 `Output=T`
fn square<T: Mul<T, Output = T>>(x: T, y: T) -> T {
    x * y
}

// 为结构体 `User` 实现方法
#[derive(Debug)]
struct User {
    name: &'static str,
    avatar_url: &'static str
}

impl User {
    // 参数为 `&self`
    // 此处 `self` 为结构体 `User` 的任意实例, `&self` 则为实例的引用
    fn show(&self) {
        println!("name: {:?}", self.name);
        println!("avatar: {:?}", self.avatar_url);
    }
}

// 使用 type 关键字定义函数指针类型别名
type MathOp = fn(i32, i32) -> i32;

fn math(op: MathOp, a: i32, b: i32) -> i32 {
    op(a, b)
}

// 函数作为参数传递
// fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
//     op(a, b)
// }

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn product(a: i32, b: i32) -> i32 {
    a * b
}

// 函数指针
fn hello() {
    println!("hello function pointer");
}

//　将函数作为返回值
fn ret_math(op: &str) -> MathOp {
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    fn product(a: i32, b: i32) -> i32 {
        a * b
    }

    match op {
        // 返回的皆为函数指针
        "sum" => sum,
        "product" => product,
        _ => {
            println!(
                "Warning: Not Implemented {:?} operator, Replace with sum",
                op
            );
            sum
        }
    }
}


// 将返回的函数和参与计算的参数直接绑定
// error[E0308]: mismatched types
// fn bind_math(op: &str, a: i32, b: i32) -> MathOp {
//     match op {
//         "sum" => sum(a, b),
//         _ => product(a, b)
//     }
// }

// 返回默认加 1 的计数函数
fn counter() -> fn(i32) -> i32 {
    fn inc(n: i32) -> i32 {
        n + 1
    }
    inc
}

// 让 `counter` 函数可以直接指定增长值 i
fn new_counter(i: i32) -> fn(i32) -> i32 {
    fn inc(n: i32) -> i32 {
        // Rust 不允许 `fn` 定义的函数 inc 不惑动态环境(counter 函数) 中的绑定变量 i
        // 因为 i 会随着栈帧的释放而释放. 如果一定要这么做, 需要使用闭包来代替
        // error[E0434]: can't capture dynamic environment in a fn item
        n + i
    }
    inc
}

fn main() {
    assert!(r#match("foo", "foobar"));

    // 按值传递的参数
    let v = vec![1, 2, 3];
    let v = modify(v);
    println!("{:?}", v);

    // 按引用传递的参数
    let mut v = vec![1, 2, 3];
    modify_ref(&mut v);
    println!("{:?}", v);

    // 作用域内的函数会屏蔽作用域外的同名函数
    f();
    {
        f();
        fn f() {
            print!("3");
        }
    }
    f();
    fn f() {
        print!("2");
    }

    // 函数参数支持模式匹配
    let s = S { i: 42 };
    f_fn(s);
    // 所有权转移
    // error[E0382]: borrow of moved value: `s`
    // println!("{:?}", s);

    foo(3);

    // 函数参数利用模式匹配来解构元组
    let t = ("Alex", 18);
    let t = swap(t);
    assert_eq!(t, (18, "Alex"));

    // 使用元组类型让函数返回多个值
    let (a, b) = addsub(5, 8);
    println!("a: {:?}, b: {:?}", a, b);

    let g = gcd(60, 40);
    assert_eq!(20, g);

    // 实现泛型函数示例
    let a: i32 = square(37, 41);
    let b: f64 = square(37.2, 41.1);
    assert_eq!(a, 1517);
    assert_eq!(b, 1528.92);

    // 使用 turbofish 操作符
    let a = square::<u32>(37, 41);
    let b = square::<f32>(37.2, 41.1);
    assert_eq!(a, 1517);
    assert_eq!(b, 1528.9199);

    // 调用为结构体 User 实现的方法
    let user = User {
        name: "Alex",
        avatar_url: "https://avatar.com/alex"
    };

    User::show(&user);
    user.show();

    // 函数本身作为参数
    let (a, b) = (2, 3);
    assert_eq!(math(sum, a, b), 5);
    assert_eq!(math(product, a, b), 6);

    // 函数指针
    // let 必须显式指定函数指针类型 `fn()`, 以及赋值使用的是函数名而非函数调用
    let fn_ptr: fn() = hello;
    println!("{:p}", fn_ptr);
    // 实际上类型是
    // `fn() {hello}`, 这是 hello 本身的类型, 而非函数指针类型
    let other_fn = hello;
    // error[E0277]: the trait bound `fn() {hello}: std::fmt::Pointer` is not satisfied
    // println!("{:p}", other_fn); // 非函数指针
    hello();
    other_fn();
    fn_ptr();
    (fn_ptr)();

    // 将函数作为返回值
    let (a, b) = (2, 3);
    let sum_fn = ret_math("sum");
    let product_fn = ret_math("product");
    let div_fn = ret_math("div");
    assert_eq!(sum_fn(a, b), 5);
    assert_eq!(product_fn(a, b), 6);
    assert_eq!(div_fn(a, b), 5);

    // 将返回的函数和参与计算的参数直接绑定
    // let (a, b) = (2, 3);
    // let sum = bind_math("sum", a, b);

    // 返回默认加 1 的计数函数
    let f = counter();
    assert_eq!(2, f(1));
    
    let f = new_counter(2);
    assert_eq!(3, f(1));
}
