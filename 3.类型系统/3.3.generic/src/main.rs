
// 泛型函数
fn foo<T>(x: T) -> T {
    return x;
}

// 泛型结构体
#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T
}

// 为泛型结构体实现具体方法
// 此处必须声明泛型 T
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        // Point {
        //     x: x,
        //     y: y
        // }
        Point {
            x,
            y,
        }
    }
}

// 标准库中的 Vec<T> 源码
// pub struct Vec<T> {
//     buf: RawVec<T>,
//     len: usize
// }

#[derive(Debug, PartialEq)]
struct Foo(i32);

#[derive(Debug, PartialEq)]
struct Bar(i32, i32);

trait Inst {
    fn new(i: i32) -> Self;
}

impl Inst for Foo {
    fn new(i: i32) -> Foo {
        Foo(i)
    }
}

impl Inst for Bar {
    fn new(i: i32) -> Bar {
        Bar(i, i + 10)
    }
}

fn foobar<T: Inst>(i: i32) -> T {
    T::new(i)
}

fn main() {
    assert_eq!(foo(1), 1);
    assert_eq!(foo("hello"), "hello");

    // 泛型结构体及其具体方法
    let point1 = Point::new(1, 2);
    let point2 = Point::new("1", "2");
    assert_eq!(point1, Point { x: 1, y: 2});
    assert_eq!(point2, Point { x: "1", y: "2"});

    // 泛型返回值的自动推导
    let f: Foo = foobar(10);
    assert_eq!(f, Foo(10));

    let b: Bar = foobar(20);
    assert_eq!(b, Bar(20, 30));
}
