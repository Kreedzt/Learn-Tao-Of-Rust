// 12-12 定义 `unless!` 宏
macro_rules! unless {
	($arg: expr, $branch: expr) => ( if !$arg { $branch };);
}

fn cmp(a: i32, b: i32) {
    unless!(a > b, {
        println!("{} < {}", a, b);
    });
}


// 12-13 使用自定义派生属性示例
#[derive(new)]
pub struct Foo;

fn main() {
    // 12-12 定义 `unless!` 宏
    let (a, b) = (1, 2);
    cmp(a, b);


    let x = Foo::new();
    assert_eq!(x, Foo);
}
