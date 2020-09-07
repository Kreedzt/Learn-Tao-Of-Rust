use std::fmt::*;

struct Duck;
struct Pig;

trait Fly {
    // 函数签名反映了该函数所有意图
    // 也可以定义默认实现
    fn fly(&self) -> bool;
}

impl Fly for Duck {
    fn fly(&self) -> bool {
        return true;
    }
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        return false;
    }
}

struct Point {
    x: i32,
    y: i32
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Point {{ x: {}, y: {}}}", self.x, self.y)
    }
}

// trait 限定
fn fly_static<T: Fly>(s: T) -> bool {
    s.fly()
}

// &Fly 是一种动态类型
fn fly_dyn(s: &Fly) -> bool {
    s.fly()
}


// T 为泛型, Option<T> 已作为标准库提供且自动引入
// enum Option<T> {
//     Some(T),
//     None
// }

// 必须实现了 Debug 才适用此方法调用
fn match_option<T: Debug>(o: Option<T>) {
    match o {
        // 当注释 Debug
        // error[E0277]: `T` doesn't implement `std::fmt::Debug`
        Some(i) => println!("{:?}", i),
        None => println!("nothing")
    }
}

fn main() {
    // 泛型
    let a: Option<i32> = Some(3);
    let b: Option<&str> = Some("hello");
    let c: Option<char> = Some('A');
    let d: Option<u32> = None;

    match_option(a);
    match_option(b);
    match_option(c);
    match_option(d);

    // trait
    let pig = Pig;
    // 静态分发
    // 编译时会被展开, 没有运行时开销
    assert_eq!(fly_static(pig), false);
    // assert_eq!(fly_static::<Pig>(pig), false);

    let duck = Duck;
    // 静态分发
    assert_eq!(fly_static(duck), true);
    // assert_eq!(fly_static::<Duck>(duck), true);

    // 动态分发, 会带来运行时开销, 在运行时查找相应类型的方法.
    assert_eq!(fly_dyn(&Pig), false);
    assert_eq!(fly_dyn(&Duck), true);

    // 手动实现Debug trait
    let origin = Point { x: 0, y: 0 };
    println!("The origin is: {:?}", origin);

    
}
