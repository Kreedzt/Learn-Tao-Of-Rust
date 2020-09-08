
// fn reset(mut arr: [u32; 5]) {
//     // |          ^^^^^^^ doesn't have a size known at compile-time
//     // 编译期无法知道大小
//     // 改为限定数组长度即可

//     arr[0]= 5;
//     arr[1] = 4;
//     arr[2] = 3;
//     arr[3] = 2;
//     arr[4] = 1;

//     println!("reset arr {:?}", arr);
// }

// 第二种方式, 胖指针(包含了动态大小类型地址信息和携带了长度信息的指针)
// C 中也有此类似语法
fn reset(arr: &mut [u32]) {

    arr[0]= 5;
    arr[1] = 4;
    arr[2] = 3;
    arr[3] = 2;
    arr[4] = 1;

    println!("reset arr {:?}", arr);
}

// 一组 0 大小类型的示例
enum Void {}
struct Foo;
struct Baz {
    foo: Foo,
    qux: (),
    baz: [u8; 0],
}

// 底类型的应用
// #![feature(never_type)]
fn foo() -> ! {
    // ...
    loop {
        println!("jh");
    }
}

// 类型推导
fn sum(a: u32, b: i32) -> u32 {
    a + (b as u32)
}

fn main() {
    // &str 的组成部分
    let str = "Hello Rust";
    let ptr = str.as_ptr();
    let len = str.len();

    println!("{:p}", ptr);
    println!("{:?}", len);

    // 将数组直接作为函数参数
    // 编译期无法知道大小
    // let arr: [u32] = [1, 2, 3, 4, 5];
    
    // let arr: [u32;5] = [1, 2, 3, 4, 5];
    // reset(arr);

    // 修改的数组并未影响原来的数组.
    // 因为 u32 类型是可复制的类型, 实现了 Copy trait, 所以整个数组也是可复制的
    // 所以当数组被传入函数中时就会被复制一份新的副本
    // print!("origin arr {:?}", arr);

    // 第二种方式: 胖指针
    let mut arr = [1, 2, 3, 4, 5];
    println!("reset before: origin array {:?}", arr);
    {
        // 可变借用
        let mut_arr: &mut[u32] = &mut arr;
        reset(mut_arr);
    }
    // 修改了原数组
    println!("reset after: origin array {:?}", arr);

    // 比较两种方式内存占用
    assert_eq!(std::mem::size_of::<&[u32;5]>(), 8);
    // 占用多了 1 倍空间, 也是称为胖指针的原因
    assert_eq!(std::mem::size_of::<&mut [u32]>(), 16);

    // 0 大小类型
    assert_eq!(std::mem::size_of::<()>(), 0);
    assert_eq!(std::mem::size_of::<Foo>(), 0);
    assert_eq!(std::mem::size_of::<Baz>(), 0);
    assert_eq!(std::mem::size_of::<Void>(), 0);
    assert_eq!(std::mem::size_of::<[(); 10]>(), 0);

    // 使用单元类型查看数据类型
    // let v: () = vec![(); 10];

    // 使用 `Vec<()>` 类型, 使用单元类型制造了一个长度为 10 的向量
    let v: Vec<()> = vec![(); 10];

    // 在一些只需要迭代次数的场合中, 使用这种方式能获得较高的性能
    // 因为 Vec 内部迭代器会针对 ZST 类型做一些优化
    for i in v {
        println!("{:?}", i);
    }

    // 底类型的应用
    let i = if false {
        // 返回 !
        foo();
    } else {
        // 返回 整数类型, 依旧可以编译通过
        100
    };

    assert_eq!(i, 100);

    // 空枚举的用法
    // let res: Result<u32, Void> = Ok(0);
    // 暂时无法编译通过
    // let Ok(num) = res;

    // 类型推导
    // 不标注, 自动推导
    let a = 1;
    let b = 2;
    assert_eq!(sum(a, b), 3);
    
    let elem = 5u8;
    // 此时为 Vec<_>
    let mut vec = Vec::new();
    // 此时为 Vec<u8>
    vec.push(elem);
    assert_eq!(vec, [5]);

    // 无法自动推导类型的情况

    let x = "1";
    // error[E0284]: type annotations required: cannot resolve `<_ as std::str::FromStr>::Err == _`
    // parse 是一个泛型方法, 当前无法自动推导
    // println!("{:?}", x.parse().unwrap());

    // 添加明确类型
    let int_x: i32 = x.parse().unwrap();
    assert_eq!(int_x, 1);
    // turbofish 操作符
    assert_eq!(x.parse::<i32>().unwrap(), 1);

    // 类型推导缺陷
    let a = 0;
    // 此时的 {integer} 并非真实类型, 只是被用于错误信息中
    // error[E0599]: no method named `is_positive` found for type `{integer}` in the current scope
    // let a_pos = a.is_positive();

    let a_pos = (a as i32).is_positive();
    println!("a: {}, a_pos: {}", a, a_pos);
    
    let b = 12i32;
    let b_pos = b.is_positive();
    println!("b: {}, b_pos: {}", b, b_pos);
}
