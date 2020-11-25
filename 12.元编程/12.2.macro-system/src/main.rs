// 12-12 定义 `unless!` 宏
// macro_rules! unless {
// 	($arg: expr, $branch: expr) => ( if !$arg { $branch };);
// }

// fn cmp(a: i32, b: i32) {
//     unless!(a > b, {
//         println!("{} < {}", a, b);
//     });
// }


// 12-13 使用自定义派生属性示例
// #[derive(new)]
// pub struct Foo;


// 12-15 普通函数示例
// fn t(i: i32) -> i32 {
//     i + 2
// }

fn main() {
    // 12-12 定义 `unless!` 宏
    // let (a, b) = (1, 2);
    // cmp(a, b);


    // let x = Foo::new();
    // assert_eq!(x, Foo);

    // t(1);


    // 12-17 另外一个较复杂的示例
    let (a, b, c, d, e) = (1, 2, 3, [4, 5], 6);
    a + b + (c + d[0]) + e;
}
