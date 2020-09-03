fn while_true(x: i32) -> i32 {
    // Rust 编译器对 while 循环体的表达式忽略
    while true {
        return x + 1;
    }
    // | |_____^ expected i32, found ()

    // 绕过编译器检测, 实际上返回的是提前 return 的值
    x
}

fn main() {
    let n = 13;

    // 条件表达式
    // n 是整数 13
    let big_n = if (n < 10 && n > -10) {
        10 * n
    } else {
        n / 2
    };

    // 自动推断为 i32, 舍去小数
    assert_eq!(big_n, 6);

    // 1..101 为迭代器
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 使用 while true
    let y = while_true(5);
    assert_eq!(y, 6);

    // match 表达式
    let number = 42;

    match number {
        // 单个值
        0 => println!("Origin"),
        // 范围
        1...3 => println!("All"),
        // 多个值
        | 5 | 7 | 13 => println!("Bad Lunk"),
        // 值绑定绑定变量, 供右侧代码使用
        n @ 42 => println!("Answer is {}", n),
        // 通配符
        _ => println!("Common"),
    }

    // if let
    let boolean = true;
    let mut binary = 0;
    if let true = boolean {
        binary = 1;
    }

    assert_eq!(binary, 1);

    // 不使用 while let
    let mut v = vec![1, 2, 3, 4, 6];

    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    // 使用 while let 简化
    while let Some(x) = v.pop() {
        println!("{}", x)
    }
}
