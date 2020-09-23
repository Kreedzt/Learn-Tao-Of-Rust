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
                },
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
}
