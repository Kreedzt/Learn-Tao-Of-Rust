use std::rc::Rc;
use std::ops::Deref;
use std::borrow::Borrow;

fn foo(s: &[i32]) {
    println!("{:?}", s[0]);
}

// 为结构体实现多个 trait 时出现同名方法的情况
struct S(i32);
trait A {
    fn test(&self, i: i32);
}

trait B {
    fn test(&self, i: i32);
}

impl A for S {
    fn test(&self, i: i32) {
        println!("From A: {:?}", i);
    }
}

impl B for S {
    fn test(&self, i: i32) {
        println!("From B: {:?}", i + 1);
    }
}

// 使用 into 方法来简化代码
#[derive(Debug)]
struct Person { name: String }
impl Person {
    // Info<String> 意味着: 实现了 into 的方法的类型都可以作为参数.
    // &str 和 String 类型都实现了 into
    // 当参数是 &str 类型时, 会通过 into 转换为 String 类型
    fn new<T: Into<String>>(name: T) -> Person {
        Person { name: name.into() }
    }
}

fn main() {
    // `String` 实现 `Deref`
    let a = "hello".to_string();
    let b = " world".to_string();
    // 当使用 + 链接操作符链接时
    // add 方法的右值参数必须是 `&str` 类型
    // 因为 String 类型实现了 `Deref<Target=str>`, 所以会自动解引用计算, 可以正常运行
    let c = a + &b;
    println!("{:?}", c); // "hello world"

    // `Vec<T>` 实现 `Deref`
    let v = vec![1, 2, 3];
    // `Vec<T>` 实现了 `Deref<Target=[T]>`
    // 自动转换为 `&[T]` 类型
    foo(&v);

    // `Rc` 指针实现 `Deref`
    let x = Rc::new("hello");
    // `Rc<T>` 实现了 Deref<Target<T>>
    // 使用起来就像 Rc 不存在一样
    println!("{:?}", x.chars());

    // 手动解引用的情况
    let x = Rc::new("hello");

    // 因为 clone 方法在 `Rc` 和 `&str` 中都被实现, 所以调用时会直接调用 `Rc` 的 clone
    let y = x.clone(); // Rc<&str>
    // 若想调用 `&str` 类型的 clone, 则需要手动解引用
    let z = (*x).clone(); // &str

    // match 时需要手动解引用
    let x = "hello".to_string();

    match x.deref() {
        "hello" => { println!("hello")},
        _ => {}
    }

    match x.as_ref() {
        "hello" => { println!("hello")},
        _ => {}
    }

    match x.borrow() {
        "hello" => { println!("hello")},
        _ => {}
    }
    
    
    match &*x {
        "hello" => { println!("hello")},
        _ => {}
    }

    match &x[..] {
        "hello" => { println!("hello")},
        _ => {}
    }

    // 原生类型使用 as 操作符转换
    let a = 1u32;
    let b = a as u64;
    let c = 3u64;
    let d = c as u32;

    // 被截断处理
    let a = std::u32::MAX;
    // 被截断
    let b = a as u16;
    assert_eq!(b, 65535);
    let e = -1i32;
    // 被截断
    let f = e as u32;
    println!("{:?}", e.abs());
    // 4294967295
    println!("{:?}", f);

    // 为结构体实现多个 trait 时出现同名方法的情况
    let s = S(1);
    // 当做 trait 静态函数使用
    A::test(&s, 1);
    B::test(&s, 1);
    // 使用 as 操作符
    <S as A>::test(&s, 1);
    <S as B>::test(&s, 1);

    // 通过 as 操作符转换类型和子类型
    let a: &'static str = "hello";
    let b: &str = a as &str;
    let c: &'static str = b as &'static str;

    // String 类型的 from 方法
    let string = "hello".to_string();
    let other_string = String::from("hello");

    // 使用 into 方法来简化代码
    let person = Person::new("Alex");
    let person = Person::new("Alex".to_string());
    println!("{:?}", person);

    // 可以使用 into 方法将 &str 类型转换为 String 类型
    let a = "hello";
    let b: String = a.into();
}
