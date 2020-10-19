// 依赖类型检查消除错误
fn sum(a: i32, b: i32) -> i32 { a + b }

fn main() {
    // error[E0308]: mismatched types
    // sum(1u32, 2u32);


    // `Vec<T>` 类型的 `insert()` 方法使用示例
    let mut vec = vec![1, 2, 3];
    vec.insert(1, 4);
    assert_eq!(vec, [1, 4, 2, 3]);
    vec.insert(4, 5);
    assert_eq!(vec, [1, 4, 2, 3, 5]);
    // thread 'main' panicked at 'insertion index (is 8) should be <= len (is 5)', library/alloc/src/vec.rs:1018:13
    // vec.insert(8, 8);

    let x = false;
    // thread 'main' panicked at 'x wasn't true!', src/main.rs:19:5
    assert!(x, "x wasn't true!");

    let a = 3;
    let b = 28;
    debug_assert!(a + b == 30, "a = {}, b = {}", a, b);
}
