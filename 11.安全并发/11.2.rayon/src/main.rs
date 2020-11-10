use rayon;
use rayon::prelude::*;

// 11-55 使用并行迭代器
fn sum_of_squares(input: &[i32]) -> i32 {
    // 并行迭代器, 返回不可变的并行迭代器类型
    input.par_iter().map(|&i| i * i).sum()
}

fn increment_all(input: &mut [i32]) {
    // 可变并行迭代器
    input.par_iter_mut().for_each(|p| *p += 1);
}

// 11-57 使用 `join()` 方法进行并行迭代
fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2));
    a + b
}

fn main() {
    // 11-55 使用并行迭代器
    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let r = sum_of_squares(&v);
    println!("{}", r);
    let mut v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    increment_all(&mut v);
    println!("{:?}", v);

    // 11-57 使用 `join()` 方法进行并行迭代
    let r = fib(32);
    assert_eq!(r, 2178309);
}
