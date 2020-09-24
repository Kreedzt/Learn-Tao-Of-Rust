use std::iter::FromIterator;
use itertools::Itertools;

// 自定义的内部迭代器
trait InIterator<T: Copy> {
    fn each<F: Fn(T) -> T>(&mut self, f: F);
}

impl<T: Copy> InIterator<T> for Vec<T> {
    fn each<F: Fn(T) -> T>(&mut self, f: F) {
        let mut i = 0;
        while i < self.len() {
            self[i] = f(self[i]);
            i += 1;
        }
    }
}

// 通过实现 Iterator trait 创建自定义迭代器
struct Counter {
    count: usize,
}

impl Iterator for Counter {
    // 指定了关联类型
    type Item = usize;
    // 关联类型影响返回类型
    fn next(&mut self) -> Option<usize> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// 自定义集合实现 `FromIterator`
#[derive(Debug)]
struct MyVec(Vec<i32>);

impl MyVec {
    fn new() -> MyVec { MyVec(Vec::new()) }

    fn add(&mut self, elem: i32) { self.0.push(elem); }
}

impl FromIterator<i32> for MyVec {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut c = MyVec::new();
        for i in iter {
            c.add(i);
        }
        c
    }
}

// 实现自定义迭代器
#[derive(Debug)]
#[must_use = "Iterator adaptors are lazy "]
pub struct Step<I> {
    iter: I,
    skip: usize,
}

// 为 `Step` 实现 `Iterator`
impl<I> Iterator for Step<I>
where
    I: Iterator,
{
    // 这里需要将关联类型 `Item` 指定为原迭代器的关联类型 `I::Item`
    type Item = I::Item;
    // 实现 `next` 和 `size_hint` 方法时, 必须符合 `Iterator trait` 中 `next` 方法签名规定的参数和返回值类型.
    // 其中 `next` 方法必须按指定的步数来迭代, 所以此处 `next` 方法实现的时候
    // 需要根据 `Step` 适配器中的 `skip` 字段来跳到相应的元素.
    // eg: 当 `skip` 是 2 的时候, 调用 `next` 时则需要跳过第一个元素, 直接跳到第二个元素
    fn next(&mut self) -> Option<I::Item> {
        let elt = self.iter.next();
        if self.skip > 0 {
            // 此处使用了 `nth` 方法, 该方法会直接返回迭代器中的第 `n` 个元素
            self.iter.nth(self.skip - 1);
        }
        elt
    }
}

// 创建 `step` 方法来产生 `Step` 迭代器
// 第一个参数为迭代器, 第二个为指定步数
pub fn step<I>(iter: I, step: usize) -> Step<I>
where
    I: Iterator,
{
    assert!(step != 0);
    Step {
        iter,
        skip: step - 1,
    }
}

// 为所有的迭代器实现 step 方法
pub trait IterExt: Iterator {
    fn my_step(self, n: usize) -> Step<Self>
    where
        Self: Sized,
    {
        step(self, n)
    }
}

impl<T: ?Sized> IterExt for T where T: Iterator {}

fn main() {
    // 自定义的内部迭代器
    let mut v = vec![1, 2, 3];
    v.each(|i| i * 3);
    assert_eq!([3, 6, 9], &v[..3]);

    // for 循环示例
    let v = vec![1, 2, 3, 4, 5];
    for i in v {
        println!("{}", i);
    }

    // fot 循环展开后的等价代码
    let v = vec![1, 2, 3, 4, 5];
    {
        // 等价于 for 循环的 scope
        let mut _iterator = v.into_iter();
        loop {
            match _iterator.next() {
                Some(i) => {
                    println!("{}", i);
                }
                None => break,
            }
        }
    }

    // 通过实现 `Iterator trait` 创建自定义迭代器
    let mut counter = Counter { count: 0 };
    assert_eq!(Some(1), counter.next());
    assert_eq!(Some(2), counter.next());
    assert_eq!(Some(3), counter.next());
    assert_eq!(Some(4), counter.next());
    assert_eq!(Some(5), counter.next());
    assert_eq!(None, counter.next());

    // 将数组转换为迭代器的 `size_hint`
    let a: [i32; 3] = [1, 2, 3];
    // 调用 `a.iter()` 使用了数组 `a` 的不可变借用, 其类型为 `&a[i32; 3]`
    // 对于 `&'a[T]` 和 `&'a mut[T]` 类型, `size_hint` 方法实际返回的是迭代器起点到终点指针的距离值
    // 此行返回的迭代器是一个结构体, 其成员包含了起始指针 `ptr` 和终点指针 `end`, 它们之间的距离就是 `size_hint` 方法返回的值
    let mut iter = a.iter();
    assert_eq!((3, Some(3)), iter.size_hint());
    iter.next();
    assert_eq!((2, Some(2)), iter.size_hint());

    // 使用迭代器来追加字符串
    let mut message = "Hello".to_string();
    message.extend(&[' ', 'R', 'u', 's', 't']);
    assert_eq!("Hello Rust", &message);

    // slice 类型数组循环示例
    // 该类型的数组使用 `for` 循环时, 并不能自动转换为迭代器, 因为并没有为 `[T]` 类型实现 `IntoIterator`
    // 而只是为 `&'a [T]` 和 `&'a mut [T]` 类型实现了 `IntoIterator`
    // 相应的 `into_iter()` 方法内部实际也分别调用了 `iter()` 和 `iter_mut()` 方法.
    // 也就是说, 在 `for` 循环中使用 `&arr` 可以自动转换为迭代器, 而无需显示地调用 `iter()` 方法
    // 用 `iter` 或 `iter_mut` 方法可以将 `slice` 类型的数组转换为 `Iter` 或 `IterMut` 迭代器
    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        println!("{:?}", i);
    }
    println!("{:?}", arr);

    // 使用可变迭代器
    let mut arr = [1, 2, 3, 4, 5];
    for i in arr.iter_mut() {
        *i += 1;
    }

    println!("{:?}", arr);

    // map 方法示例
    let a = [1, 2, 3];
    let mut iter = a.into_iter().map(|x| 2 * x);
    println!("{:?}", a);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), None);

    // 部分迭代器适配器使用示例
    let arr1 = [1, 2, 3, 4, 5];
    let c1 = arr1.iter().map(|x| 2 * x).collect::<Vec<i32>>();
    assert_eq!(&c1[..], [2, 4, 6, 8, 10]);

    let arr2 = ["1", "2", "3", "h"];
    let c2 = arr2
        .iter()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<i32>>();
    assert_eq!(&c2[..], [1, 2, 3]);

    let arr3 = ['a', 'b', 'c'];
    for (idx, val) in arr3.iter().enumerate() {
        println!("idx: {:?}, val: {}", idx, val.to_uppercase());
    }

    // rev 方法示例
    let a = [1, 2, 3];
    let mut iter = a.iter().rev();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);

    // `next_back` 方法使用示例
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let mut iter = numbers.into_iter();
    assert_eq!(Some(1), iter.next());
    // 两次反向遍历
    assert_eq!(Some(6), iter.next_back());
    assert_eq!(Some(5), iter.next_back());
    // 执行
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(4), iter.next());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next_back());

    // `any` 和 `fold` 的使用示例
    let a = [1, 2, 3];
    // 注意: `any` 和 `fold` 传入的闭包参数是一个引用
    // 使用数组的 `iter` 方法, 创建的迭代器是 `Iter` 类型
    // 该类型的 `next` 方法返回的是 `Option<&[T]>` 或 `Option<&mut[T]>` 的值
    // 而 `for` 循环实际上是一个语法糖, 会自动调用迭代器的 `next` 方法
    // `for` 循环中的循环变量则是通过模式匹配,
    // 从 `next` 返回的 `Option<&[T]>` 或 `Option<&mut[T]>` 类型中获取 `&[T]` 或 `Option<&mut [T]>` 类型中获取 `&[T]` 或 `&mut [T]` 类型的值的
    // 所以参数仅引用类型
    assert_eq!(a.iter().any(|&x| x != 2), true);
    let sum = a.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);

    // `any` 方法示意
    let arr = [1, 2, 3];
    let result1 = arr.iter().any(|&x| x != 2);
    let result2 = arr.iter().any(|x| *x != 2);
    // error[E0277]: can't compare `&{integer}` with `{integer}`
    // let result2 = arr.iter().any(|x| x != 2);
    assert_eq!(result1, true);

    // 使用 `fold` 对数组求和示例
    let arr = vec![1, 2, 3];
    let sum1 = arr.iter().fold(0, |acc, x| acc + x);
    let sum2 = arr.iter().fold(0, |acc, x| acc + *x);
    let sum3 = arr.iter().fold(0, |acc, &x| acc + x);
    // `into_iter` 会获取所有权, 且迭代器的 `next` 方法返回的是 `Option<T>` 类型
    // 循环拿到的是值, 不是引用
    let sum4 = arr.into_iter().fold(0, |acc, x| acc + x);
    // error[E0308]: mismatched types
    // let sum4 = arr.into_iter().fold(0, |acc, &x| acc + x);
    // error[E0614]: type `{integer}` cannot be dereferenced
    // let sum4 = arr.into_iter().fold(0, |acc, x| acc + *x);
    assert_eq!(sum1, 6);
    assert_eq!(sum2, 6);
    assert_eq!(sum3, 6);
    assert_eq!(sum4, 6);

    // 自定义集合 `MyVec` 实现 `FromIterator`
    let iter = (0..5).into_iter();
    let c = MyVec::from_iter(iter);
    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);

    let iter = (0..5).into_iter();
    let c: MyVec = iter.collect();
    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);

    let iter = (0..5).into_iter();
    let c = iter.collect::<MyVec>();
    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);

    // 应用迭代器适配器 `Step`
    let arr = [1, 2, 3, 4, 5, 6];
    // [1, 3, 5] => ...
    let sum = arr.iter().my_step(2).fold(0, |acc, x| acc + x);
    assert_eq!(9, sum);

    // 使用 `Iteratortools`
    let data = vec![1, 2, 3, 3, 4, 6, 7, 9];
    // 仅拿到位置 mod 3 为 0 的数据的迭代器
    let r = data.iter().positions(|v| v % 3 == 0);
    let rev_r = data.iter().positions(|v| v % 3 == 0).rev();

    for i in r {
        println!("{:?}", i);
    }

    println!("====");

    for i in rev_r {
        println!("{:?}", i);
    }
}
