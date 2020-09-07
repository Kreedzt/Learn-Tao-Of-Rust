/// # 文档注释: Sum 函数
/// 该函数为求和函数
/// # usage:
/// assert_eq!(3, sum(1, 2));
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // 这是单行注释的示例
    /*
    * 区块注释, 被包含的区域都会被注释
    * 可以把 /* 区块 */ 置于 代码中的任何位置
     */
    /*
    注释区块中的 * 实际不需要
     */
    let x = 5 + /*  90 + */ 5;
    assert_eq!(x, 10);
    println!("2 + 3 = {}", sum(2, 3));

    let num = 2;
    let num2 = 10e2;
    let num3 = 0xa3;
    let pt = Box::new(3);

    // println! 各格式化形式
    println!("{}", num);
    println!("{:?}", num);
    println!("{:o}", num3);
    println!("{:x}", num3);
    println!("{:X}", num3);
    println!("{:p}", pt);
    println!("{:b}", num);
    println!("{:e}", num2);
    println!("{:E}", num2);
}
