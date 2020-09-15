// 引用移动语义类型作为函数参数
fn foo(s: String) -> String {
    let w = " world".to_string();
    s + &w
}

fn main() {
    let a = "hello".to_string();
    // 重新绑定
    let b = a;

    // 绑定默认不可变
    let x = "hello".to_string();
    // error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
    // x += " world";

    let mut x = "hello".to_string();
    x += " world";
    assert_eq!("hello world", x);

    // let 默认创建词法作用域
    // 生命周期就是整个 main 作用域.
    // 作为栈变量, a 会随着 main 函数栈帧的销毁而被清理.
    // 并且, 这是编译器可以知道的事实
    let a = "hello";
    let b = "rust";
    let c = "world";
    // 字符串字面量实现了按位复制
    let d = c;
    println!("{}", c);

    // 使用花括号创建词法作用域
    let outer_val = 1;
    let outer_sp = "hello".to_string();
    {
        let inner_val = 2;
        // 复制语义类型, 不会转移所有权
        outer_val;
        outer_sp;
    }
    println!("{:?}", outer_val);
    // error[E0425]: cannot find value `inner_val` in this scope
    // println!("{:?}", inner_val);
    // error[E0382]: borrow of moved value: `outer_sp`
    // println!("{:?}", outer_sp);

    // match 匹配会产生新的词法作用域
    let a = Some("hello".to_string());

    // String 类型具有移动语义
    match a {
        // 独立词法作用域
        Some(s) => println!("{:?}", s),
        _ => println!("nothing")
    }

    // error[E0382]: borrow of partially moved value: `a`
    // println!("{:?}", a);

    // 移动语义类型
    let v = vec![1, 2, 3];

    // 进入 for 循环时已经转移了所有权
    for i in v {
        println!("{:?}", i);
        // error[E0382]: borrow of moved value: `v`
        // println!("{:?}", v);
    }

    // if let 块创建新的作用域
    let a = Some("hello".to_string());
    if let Some(s) = a {
        println!("{:?}", s);
    }
    // error[E0382]: borrow of partially moved value: `a`
    // println!("{:?}", a);

    // while let 块创建新的作用域
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `${:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }

    // let a = Some("hello".to_string());
    let a = Some(vec![1, 2]);
    // error[E0382]: use of moved value
    // while let Some(i) = a {
    //     println!("{:?}", i);
    // }
    // println!("{:?}", a);


    // 函数
    let s = "hello".to_string();
    let ss = foo(s);
    // error[E0382]: borrow of moved value: `s`
    // println!("{:?}", s);

    // 闭包
    let s = "hello".to_string();
    let join = |i: &str| { s + i };
    assert_eq!("hello world", join(" world"));
    // error[E0382]: borrow of moved value: `s`
    // println!("{:?}", s);
}
