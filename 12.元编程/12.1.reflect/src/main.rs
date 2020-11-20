use std::any::{Any, TypeId};

#[derive(Debug)]
enum E {
    H,
    He,
    Li,
}
struct S {
    x: u8,
    y: u8,
    z: u16,
}

// 注意: 因为参数类型是 trait 对象, 所以必须为引用
fn print_any(a: &Any) {
    // 对转换结果匹配, 成功则打印相关内容
    if let Some(v) = a.downcast_ref::<u32>() {
        println!("u32 {:x}", v);
    } else if let Some(v) = a.downcast_ref::<E>() {
        println!("enum E {:?}", v);
    } else if let Some(v) = a.downcast_ref::<S>() {
        println!("struct s {:x} {:x} {:x}", v.x, v.y, v.z);
    } else {
        println!("else!");
    }
}


// 12-7 使用 `Box<Any>`
// 注意: `Box<Any>` 类型是独占所有权类型
fn print_if_string(value: Box<Any>) {
    if let Ok(string) = value.downcast::<String>() {
        println!("String (length {}): {}", string.len(), string);
    } else {
        println!("Not String");
    }
}

fn main() {
    // 12-3 使用 is 判断类型
    let v1 = 0xc0ffee_u32;
    let v2 = E::He;
    let v3 = S {
        x: 0xde,
        y: 0xad,
        z: 0xbeef,
    };
    let v4 = "rust";

    let mut a: &Any;
    a = &v1;
    assert!(a.is::<u32>());
    println!("{:?}", TypeId::of::<u32>());

    a = &v2;
    assert!(a.is::<E>());
    println!("{:?}", TypeId::of::<E>());

    a = &v3;
    assert!(a.is::<S>());
    println!("{:?}", TypeId::of::<S>());

    a = &v4;
    assert!(a.is::<&str>());
    println!("{:?}", TypeId::of::<&str>());


    // 12-5 使用 `downcast_ref` 向下转换类型
    print_any(&0xc0ffee_u32);
    print_any(&E::He);
    print_any(&S {
        x: 0xde,
        y: 0xad,
        z: 0xbeef,
    });
    print_any(&"rust");
    print_any(&"hoge");


    // 12-7 使用 `Box<Any>`
    let my_string = "Hello World".to_string();
    print_if_string(Box::new(my_string));
    print_if_string(Box::new(0i8));
}
