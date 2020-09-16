// 将数组作为函数参数传递
// 可变数组
// 函数签名也支持模式匹配, 相当于使用 let 将 v 重新声明了可变绑定
fn foo(mut v: [i32;3]) -> [i32;3] {
    v[0] = 3;
    assert_eq!([3, 2, 3], v);
    v
}

// 使用引用作为函数参数
// 前提: 出借所有权的绑定必须是一个可变绑定
// 借用, 调用完毕后归还所有权
fn foo2(v: &mut [i32; 3]) {
    v[0] = 3;
}

// 冒泡排序
fn bubble_sort(a: &mut Vec<i32>) {
    let mut n = a.len(); // 获取向量长度

    while n > 0 {
        // 初始化遍历游标, max_ptr 始终指向最大值
        let (mut i, mut max_ptr) = (1, 0);
        // 冒泡开始
        while i < n {
            if a[i - 1] > a[i] {
                // 交换位置
                a.swap(i - 1, i);
                max_ptr = i;
            }

            i += 1;
        }
        // 本次遍历的最大值位置也是下一轮冒泡的终点
        n = max_ptr;
    }
}

fn compute(input: &u32, output: &mut u32) {
    if *input > 10 {
        *output = 1;
    }

    if *input > 5 {
        *output *= 2;
    }
}

// 优化 compute 函数
// 因为两个参数分为可变与不可变, 所以不可能指向同一块内存
fn compute_optimized(input: &u32, output: &mut u32) {
    let cached_input = *input;
    if cached_input > 10 {
        *output = 2;
    } else if cached_input > 5 {
        *output *= 2;
    }
}

// 解引用 `&String` 类型
fn join(s: &String) -> String {
    // 不允许将借用 s 的所有权转移, 否则会出现野指针
    // error[E0507]: cannot move out of `*s` which is behind a shared reference
    let append = *s;
    "hello".to_string() + &append
}

fn main() {
    // 基本数字类型的数组, v 为复制语义
    // 不可变数组
    let v = [1, 2, 3];
    // 传入为按位复制
    foo(v);
    assert_eq!([1, 2, 3], v);

    // 使用引用作为函数参数
    let mut v = [1, 2, 3];
    foo2(&mut v);
    assert_eq!([3, 2, 3], v);

    // 调用冒泡排序
    let mut a = vec![1, 4, 5, 3, 2];
    bubble_sort(&mut a);
    println!("{:?}", a);

    // 借用检查保障了内存安全
    let i = 20;
    let mut o = 5;
    compute(&i, &mut o);
    println!("o: {}", o);

    // 假设不存在 Rust 的借用检查, compute 函数可能存在的问题
    // 将会输出 1, 不在预期结果
    let mut i = 20;
    // error[E0502]: cannot borrow `i` as mutable because it is also borrowed as immutable
    // compute(&i, &mut i);

    // 优化 compute 函数
    let i = 20;
    let mut o = 5;
    compute_optimized(&i, &mut o);

    // 解引用 `&String` 类型
    let x = " hello".to_string();
    join(&x);
}
