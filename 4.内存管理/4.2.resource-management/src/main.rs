use std::ops::Drop;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct S(i32);

// 为结构体实现 Drop
impl Drop for S {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

// 使用 Box<T> 指针分配堆内存
fn create_box() {
    let box3 = Box::new(3);
}

// 链表节点指针
// type NodePtr<T> = Option<Box<Node<T>>>;

// 使用 Rc
// type NodePtr<T> = Option<Rc<Node<T>>>;

// 使用 RefCell + Rc
type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

// 链表节点 Node 结构体
struct Node<T> {
    data: T,
    next: NodePtr<T>
}

// 实现 Drop 检测内存释放
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

// 构造悬垂指针
// fn foo<'a>() -> &'a str {
//     let a = "hello".to_string();
//     // error[E0515]: cannot return reference to local variable `a`
//     // 局部变量 a 在离开 foo 后被销毁. 如果把该引用传递到函数外面就会出现问题
//     &a
// }

struct A {
    a: u32,
    b: Box<u64>
}

struct B(i32, f64, char);
struct N;
enum E {
    H(u32),
    M(Box<u32>)
}

union U {
    u: u32,
    v: u64
}

fn main() {
    // 检测未初始化变量
    let x: i32;
    // error[E0381]: borrow of possibly-uninitialized variable: `x`
    // println!("{}", x);

    // 检测分支流程是否产生为初始化变量
    // if 语句中初始化变量

    let x: i32;

    // 编译器的静态分支流程分析并不能识别 if 表达式中的条件是 true
    // 所以要检查所有的情况
    if true {
        x = 1;
    }
    // 去掉 else 编译报错
    // else {
    //     x = 2;
    // }

    // error[E0381]: borrow of possibly-uninitialized variable: `x`
    // 去掉 else 后, 注释打印代码也正常, 因为未使用变量
    // println!("{}", x);

    // 在 loop 循环中使用 break 关键字
    let x: i32;
    loop {
        if true {
            x = 2;
            break;
        }
    }

    println!("{}", x);

    // 绑定空数组向量
    // 不加显示标注会编译报错
    // error[E0282]: type annotations needed for `Vec<T>`
    // let a = vec![];
    // error[E0282]: type annotations needed for `[_; 0]`
    // let b = [];
    
    let a: Vec<i32> = vec![];
    let b: [i32; 0] = [];

    // 将已初始化变量绑定给另外一个变量
    let x = 42;
    let y = Box::new(5);
    // 打印输出指针地址
    println!("{:p}", y);

    let x2 = 2;
    let y2 = y;
    // error[E0382]: borrow of moved value: `y`
    // println!("{:p}", y);

    // String 和 Vec 也是一种智能指针
    let s = String::from("hello");
    // error[E0277]: the size for values of type `str` cannot be known at compilation time
    // let deref_s: str = *s;
    let v = vec![1, 2, 3];
    // error[E0277]: the size for values of type `[u32]` cannot be known at compilation time
    // let deref_v: [u32]= *v;

    // 结构体实现 Drop
    let x = S(1);
    println!("crate x: {:?}", x);
    {
        let y = S(2);
        println!("crate y: {:?}", y);
        println!("exit inner scope");
    }

    println!("exit main");

    // 使用 Box<T> 指针来分配堆内存
    let box1 = Box::new(1);
    {
        let box2 = Box::new(2);
    }

    for _ in 0..1_000 {
        create_box();
    }

    let mut v = vec![1, 2, 3];
    {
        // 变量被置于显式内部作用域, 离开会自动清理
        v
    };

    // error[E0382]: borrow of moved value: `v`
    // v.push(4);

    let x = S(1);
    println!("create x: {:?}", x);
    // 变量遮蔽并不会导致生命周期提前结束
    let x = S(2);
    println!("create shadowing x: {:?}", x);

    // 尝试实现链表
    // let mut first = Box::new(Node { data: 1, next: None });
    // let mut second = Box::new(Node { data: 2, next: None });
    // error[E0382]: assign to part of moved value: `*second`
    // first.next = Some(second);
    // 因使用指针 `Box<T>` 发生了值移动, second 变成了未初始化变量
    // second.next = Some(first);

    // 对链表使用 Rc
    // let first = Rc::new(Node { data: 1, next: None });
    // 循环链表
    // let second = Rc::new(Node { data: 2, next: Some(first.clone())});
    // 不能对不可变指针修改
    // first.next = Some(second.clone());
    // second.next = Some(first.clone());

    // 使用 `RefCell<T>` 保证内部可变
    let first = Rc::new(RefCell::new(Node {
        data: 1,
        next: None
    }));

    let second = Rc::new(RefCell::new(Node {
        data: 2,
        next: Some(first.clone())
    }));

    // 制造循环引用
    // 循环引用后析构函数并没有运行, 存在内存泄露
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(first.clone());

    // 悬垂指针
    // let x = foo();

    // 验证三种符合结构内存布局
    println!("Box<u32>: {:?}", std::mem::size_of::<Box<u32>>());
    println!("A: {:?}", std::mem::size_of::<A>());
    println!("B: {:?}", std::mem::size_of::<B>());
    println!("N: {:?}", std::mem::size_of::<N>());
    println!("E: {:?}", std::mem::size_of::<E>());
    println!("U: {:?}", std::mem::size_of::<U>());
}
