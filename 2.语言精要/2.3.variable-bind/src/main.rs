pub fn temp() -> i32 {
    return 1;
}

fn main() {
    // 位置表达式与值表达式
    let x = &temp();
    // error[E0070]: invalid left-hand side expression
    // temp() = *x;

    // 不可变绑定与可变绑定
    let a = 1;
    // a = 2; // immutable and error

    let mut b = 2;
    b = 3; // mutable

    let place1 = "hello";
    let place2 = "hello".to_string();
    let other = place1;
    println!("{:?}", other);

    let other = place2;
    println!("{:?}", other);

    // 引用操作
    let a = [1, 2, 3];
    let b = &a;
    println!("{:p}", b);
    
    let mut c = vec![1, 2, 3];
    let d = &mut c;
    d.push(4);
    println!("{:?}", d);

    let e = &42;
    assert_eq!(42, *e);
}
