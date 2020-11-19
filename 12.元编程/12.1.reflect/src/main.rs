use std::any::{Any, TypeId};

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
}
