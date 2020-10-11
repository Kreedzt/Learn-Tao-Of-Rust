use std::cmp::Ordering;

struct Foo;

// `match` 匹配 array 数组示例
// 接收定长数组
fn pick(arr: [i32; 3]) {
    match arr {
        // 通过匹配数组的不同元素, 可以实现特定的功能
        [_, _, 3] => println!("ends with 3"),
        [a, 2, c] => println!("{:?}, 2, {:?}", a, c),
        // 最后一个分支必须使用通配符或其他变量来穷尽枚举
        [_, _, _] => println!("pass!"),
    }
}

// `match` 匹配数组切片示例
fn sum(num: &[i32]) {
    match num {
        [one] => println!(" at least two"),
        [first, second] => println!("{:?} + {:?} = {:?}", first, second, first + second),
        _ => println!("sum is {:?}", num.iter().fold(0, |sum, i| sum + i)),
    }
}

fn main() {
    // 创建空数组: 实际上未分配堆内存.
    // 如果整个函数中都未为其填充元素, 则编译器认定他为未初始化内存, 报错.
    let mut vec = Vec::new();
    // 此时编译器推断为 `i32` 类型
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);
    // vec[10]; // 越界访问: panic

    // 弹出, FILO
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);
    vec[0] = 7;
    assert_eq!(vec.get(0), Some(&7));
    // 越界访问
    assert_eq!(vec.get(10), None);

    // 使用 `extend` 追加元素
    vec.extend([1, 2, 3].iter().cloned());
    assert_eq!(vec, [7, 1, 2, 3]);
    assert_eq!(vec.get(0..2), Some(&[7, 1][..]));
    let mut vec2 = vec![4, 5, 6];
    // 使用 `append` 追加数组
    vec.append(&mut vec2);
    assert_eq!(vec, [7, 1, 2, 3, 4, 5, 6]);
    assert_eq!(vec2, []);
    // 交换元素
    vec.swap(1, 3);
    assert!(vec == [7, 3, 2, 1, 4, 5, 6]);

    let slice = [1, 2, 3, 4, 5, 6, 7];
    // 从切片全部替换数组
    vec.copy_from_slice(&slice);
    assert_eq!(vec, slice);
    let slice = [4, 3, 2, 1];
    // 效果与上等价, 但是该方法支持实现 `Clone` 的类型元素
    // vec.clone_from_slice(&slice);
    // assert_eq!(vec, slice);

    // 使用 `with_capatity` 预分配堆内存来创建数组
    let mut vec = Vec::with_capacity(10);
    for i in 0..10 {
        vec.push(i);
    }
    // 从索引 0 开始截断, 效果等同于 `clear()`
    vec.truncate(0);
    assert_eq!(10, vec.capacity());

    for i in 0..10 {
        vec.push(i);
    }
    vec.clear();
    assert_eq!(10, vec.capacity());
    // 释放预分配的堆内存
    // 实际上: 该方法只有在 `vec` 数组中元素被清空之后才会释放预分配的堆内存
    // 若未占满, 就会压缩未被使用的那部分容量空间, 相当于重新分配堆内存
    vec.shrink_to_fit();
    assert_eq!(0, vec.capacity());

    for i in 0..10 {
        vec.push(i);
        // 容量成倍申请
        print!("{:?}/", vec.capacity());
    }

    // `Vector` 数组存储零大小类型示例
    // 该数组本质是一个智能指针, 跟 `String` 类型的字符串一样, 也由 3 部分组成
    // - 指向堆中字节序列的指针(`as_ptr` 方法)
    // - 记录堆中字节序列的字节长度(`len` 方法)
    // - 堆分配的容量(`capacity` 方法)
    // 因为此时并未预分配堆内存, 所以其内部指针并非指向堆内存, 但也是不是空指针
    let mut vec = Vec::new();
    vec.push(Foo);
    // 用一个实际不可能分配的最大值来表示 ZST 的容量
    assert_eq!(vec.capacity(), std::usize::MAX);

    // `contains` 等方法使用示例
    let v = [10, 40, 30];
    assert!(v.contains(&30));
    assert!(!v.contains(&50));
    assert!(v.starts_with(&[10]));
    assert!(v.starts_with(&[10, 40]));
    assert!(v.ends_with(&[30]));
    assert!(v.ends_with(&[40, 30]));
    assert!(v.ends_with(&[]));
    let v: &[u8] = &[];
    assert!(v.starts_with(&[]));
    assert!(v.ends_with(&[]));

    // `binary_search` 系列泛型方法使用示例
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    // 传递引用类型
    // 返回 Result 类型的索引值
    assert_eq!(s.binary_search(&13), Ok(9));
    assert_eq!(s.binary_search(&4), Err(7));
    let r = s.binary_search(&1);
    assert!(match r {
        Ok(1...4) => true,
        _ => false,
    });

    let seek = 13;
    // 传递一个闭包, 返回 `Ordering` 枚举
    // `cmp` 方法是 `Ord trait` 中定义的, 该方法只能用于检索实现了 `Ord` 的类型
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));

    // 以元组的第二位进行排序的有序数组
    let s = [
        (0, 0),
        (2, 1),
        (4, 1),
        (5, 1),
        (3, 1),
        (1, 2),
        (2, 3),
        (4, 5),
        (5, 8),
        (3, 13),
        (1, 21),
        (2, 32),
        (4, 55),
    ];
    // 按元组第二位来设置检索条件
    assert_eq!(s.binary_search_by_key(&13, |&(a, b)| b), Ok(9));

    // sort 方法使用示例
    let mut v = [-5i32, 4, 1, -3, 2];
    v.sort();
    assert!(v == [-5, -3, 1, 2, 4]);
    v.sort_by(|a, b| a.cmp(b));
    assert!(v == [-5, -3, 1, 2, 4]);
    v.sort_by(|a, b| b.cmp(a));
    assert!(v == [4, 2, 1, -3, -5]);
    v.sort_by_key(|k| k.abs());
    assert!(v == [1, 2, -3, 4, -5]);

    // 比较操作示例
    // 浮点数只能用偏序比较
    let result = 1.0.partial_cmp(&2.0);
    assert_eq!(result, Some(Ordering::Less));

    // 整数满足全序关系
    let result = 1.cmp(&1);
    assert_eq!(result, Ordering::Equal);

    // 字符串满足偏序关系, 默认字典序(也就是按字符串首字母进行比较)
    let result = "abc".partial_cmp(&"Abc");
    assert_eq!(result, Some(Ordering::Greater));

    let mut v: [f32; 5] = [5.0, 4.1, 1.2, 3.4, 2.5];
    // `sort_by` 按照返回结果是否等于 `Less` 的规则进行排序的
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(v == [1.2, 2.5, 3.4, 4.1, 5.0]);

    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
    assert!(v == [5.0, 4.1, 3.4, 2.5, 1.2]);

    // `match` 匹配 array 数组示例
    let arr = [1, 2, 3];
    pick(arr);
    let arr = [1, 2, 5];
    pick(arr);
    let arr = [1, 3, 5];
    pick(arr);


    // `match` 匹配数组切片示例
    sum(&[1]);
    sum(&[1, 2]);
    sum(&[1, 2, 3]);
    sum(&[1, 2, 3, 5]);
}
