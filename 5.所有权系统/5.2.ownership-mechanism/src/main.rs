// #[derive(Debug)]
// struct A {
//     // 结构体 A 的成员都为复制语义类型
//     a: i32,
//     b: u32
// }


// 手动为结构体 A 实现 Copy, 实现按位复制
#[derive(Debug, Copy, Clone)]
struct A {
    // 结构体 A 的成员都为复制语义类型
    a: i32,
    b: u32
}

fn main() {
    let x = Box::new(5);
    // 具有引用语义的 `Box<T>` 会移动
    let y = x;
    // println!("{:?}", x);

    let a = A { a: 1, b: 2 };
    // 所有权转移
    // 实现复制后, 按位复制
    let b = a;
    // error[E0382]: borrow of moved value: `a`
    println!("{:?}", a);

    // 元组中有引用语义类型
    let a = ("a".to_string(), "b".to_string());
    let b = a;
    // error[E0382]: borrow of moved value: `a`
    // println!("{:?}", a);
    // 元组中没有引用语义类型, 自动实现 Copy
    let c = (1, 2, 3);
    let d = c;
    println!("{:?}", c);
}
