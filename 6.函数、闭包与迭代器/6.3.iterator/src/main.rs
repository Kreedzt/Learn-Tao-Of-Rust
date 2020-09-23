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
}
