// 函数示例
pub fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        return "fizzbuzz".to_string();
    } else if num % 3 == 0 {
        return "fizz".to_string();
    } else if num % 5 == 0 {
        return "buzz".to_string();
    } else {
        return num.to_string();
    }
}

// 函数作为参数
pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn product(a: i32, b: i32) -> i32 {
    a * b
}

fn is_true() -> bool { true }

// 函数作为返回值
fn true_maker() -> fn() -> bool { is_true }

// CTFE
// 受限: 必须可以确定值
const fn init_len() -> usize {
    return 5;
}

// 闭包作为参数
// 参数是一个泛型 F: 且接受 Fn() -> i32 的限定
// 代表该函数只允许实现 Fn() -> i32 trait 的类型作为参数
fn closure_math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

// 闭包作为返回值
// 返回值实现了 Fn(i32) -> i32 的类型
// 函数定义时不知道具体的返回类型, 但是在函数调用时, 编译器会推断出来, 这个过程是 0 成本抽象, 发生在编译期
fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;

    // 使用 move, 所有权转移(针对 i)
    move |j| j * i
}

fn main() {
    // 函数示例
    assert_eq!(fizz_buzz(15), "fizzbuzz".to_string());
    assert_eq!(fizz_buzz(3), "fizz".to_string());
    assert_eq!(fizz_buzz(5), "buzz".to_string());
    assert_eq!(fizz_buzz(13), "13".to_string());

    // 词法作用域示例
    let v = "hello world!";
    assert_eq!(v, "hello world!");
    // 变量遮蔽
    let v = "hello Rust!";
    assert_eq!(v, "hello Rust!");
    {
        // 变量遮蔽
        let v = "hello World!";
        assert_eq!(v, "hello World!");
    }
    assert_eq!(v, "hello Rust!");

    // 函数作为参数
    let a = 2;
    let b = 3;
    assert_eq!(math(sum, a, b), 5);
    assert_eq!(math(product, a, b), 6);

    // 函数作为返回值
    assert_eq!(true_maker()(), true);

    // CTFE
    let arr = [0; init_len()];

    // 闭包示例
    let out = 42;

    // 不是闭包, 无法访问外部变量
    // fn add(i: i32, j: i32) -> i32 { i + j + out }
    fn add(i: i32, j: i32) -> i32 { i + j }

    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out };
    let closure_inferred = |i, j| i + j + out;

    let i = 1;
    let j = 2;
    assert_eq!(3, add(i, j));
    assert_eq!(45, closure_annotated(i, j));
    assert_eq!(45, closure_inferred(i, j));

    // 闭包作为参数
    let a = 2;
    let b = 3;
    assert_eq!(closure_math(|| a + b), 5);
    assert_eq!(closure_math(|| a * b), 6);
}
