#[derive(Copy, Clone)]
struct A {
    a: i32,
    b: Box<i32>,
    //      ----------- this field does not implement `Copy`
}

fn main() {
    // 编译器对原生类型进行按位复制
    let x = 5;
    let y = x;

    assert_eq!(x, 5);
    assert_eq!(y, 5);
}
