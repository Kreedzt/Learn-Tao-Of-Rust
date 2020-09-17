use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::cell::{RefCell, Cell};
use std::borrow::Cow;
use std::thread;


struct Node {
    next: Option<Rc<RefCell<Node>>>,
    head: Option<Weak<RefCell<Node>>>
}

impl Drop for Node {
    fn drop(&mut self) {
        // 正常调用, 不存在内存泄露问题
        println!("Dropping!");
    }
}

// 使用 `Cell<T>` 实现字段级可变
struct Foo {
    x: u32,
    y: Cell<u32>
}

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}

fn abs_sum(ns: &[i32]) -> i32 {
    let mut lst = Cow::from(ns);
    abs_all(&mut lst);
    // 类似 js 上 Array 的 reduce
    lst.iter().fold(0, |acc, n| acc + n)
}

// 利用 `Cow<T>` 来统一实现规范
#[derive(Debug)]
struct Token<'a> {
    raw: Cow<'a, str>,
}

impl<'a> Token<'a> {
    pub fn new<S>(raw: S) -> Token<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Token {
            // 不论 `String` 还是 `&str`, 都通过 `into()` 转换
            // 并且对于 `String` 和 `&str` 还可以跨线程安全传递
            raw: raw.into()
        }
    }
}

fn main() {
    // 智能指针时 `Box<T>` 独占所有权
    let x = Box::new("hello");
    let y = x;
    // error[E0382]: borrow of moved value: `x`
    // println!("{:?}", x);

    // 解引用智能指针
    // 实现了按位复制
    let a = Box::new("hello");
    // 引用语义, 必须转义所有权
    let b = Box::new("Rust".to_string());
    let c = *a;
    let d = *b;
    // let c = *(a.deref());
    // let d = *(b.deref());
    println!("{:?}", a);
    // error[E0382]: borrow of moved value: `b`
    // println!("{:?}", b);

    let r = Rc::new("Rust".to_string());
    let a = Arc::new(vec![1.0, 2.0, 3.0]);
    // error[E0507]: cannot move out of an `Rc`
    // let x = *r;
    println!("{:?}", r);
    // error[E0507]: cannot move out of an `Arc`
    // let f = *a;

    // `Rc<T>` 示例
    let x = Rc::new(45);
    let y1 = x.clone(); // 增加强引用计数
    let y2 = x.clone(); // 增加强引用计数
    println!("{:?}", Rc::strong_count(&x));
    let w = Rc::downgrade(&x); // 增加弱引用计数
    println!("{:?}", Rc::weak_count(&x));

    let y3 = &*x; // 不增加计数
    println!("{}", 100 - *x);

    // 利用 `Weak<T>` 解决循环引用的内存泄露问题
    let first = Rc::new(RefCell::new(Node { next: None, head: None }));
    let second = Rc::new(RefCell::new(Node { next: None, head: None }));

    let third = Rc::new(RefCell::new(Node { next: None, head: None }));
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(third.clone());
    third.borrow_mut().head = Some(Rc::downgrade(&first));

    // 使用 `Cell<T>` 实现字段级可变
    let foo = Foo { x: 1, y: Cell::new(3) };
    assert_eq!(1, foo.x);
    assert_eq!(3, foo.y.get());
    foo.y.set(5);
    assert_eq!(5, foo.y.get());

    // `RefCell<T>` 内部可变性示例
    let x = RefCell::new(vec![1, 2, 3, 4]);
    println!("{:?}", x.borrow());
    x.borrow_mut().push(5);
    println!("{:?}", x.borrow());

    // 违反 `RefCell<T>` 运行时借用规则, 引发线程 panic
    let x = RefCell::new(vec![1, 2, 3, 4]);
    let mut mut_v = x.borrow_mut();
    mut_v.push(5);
    // thread 'main' panicked at 'already borrowed: BorrowMutError', src/main.rs:89:24
    // let mut mut_v2 = x.borrow_mut();
    
    // `Cow<T>` 示例
    let s1 = [1, 2, 3];
    // 切片
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);
    println!("IN: {:?}", s1);
    println!("OUT: {:?}", i1);

    // `Vec<T>` 是有所有权的类型, 调用 `to_mut()` 方法并不会克隆新对象
    let mut v1 = Cow::from(vec![1, 2, -3, 4]);
    abs_all(&mut v1);
    println!("IN/OUT: {:?}", v1);
    
    let s3 = [1, 3, 5, 6];
    // 不会调用 `to_mut()`
    let sum1 = abs_sum(&s3[..]);
    println!("{:?}", s3);
    println!("{}", sum1);

    let s4 = [1, -3, 5, -6];
    // 调用 `to_mut()`, 克隆数据
    let sum2 = abs_sum(&s4[..]);
    println!("{:?}", s4);
    println!("{}", sum2);

    // 利用 `Cow<T>` 来统一实现规范
    let token = Token::new("abc123");
    let token = Token::new("api.example.io".to_string());
    thread::spawn(move || {
        println!("token: {:?}", token);
    }).join().unwrap();

    // 无法跨线程传递动态字符串切片
    let raw = String::from("abc");
    // error[E0597]: `raw` does not live long enough
    // let s = &raw[..];
    // let token = Token::new(s);
    // thread::spawn(move || {
    //     println!("token: {:?}", token);
    // }).join().unwrap();
}
