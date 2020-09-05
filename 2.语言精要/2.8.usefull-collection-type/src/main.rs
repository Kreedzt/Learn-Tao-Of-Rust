// use std::collections::VecDeque;
// use std::collections::LinkedList;
// use std::collections::HashMap;
use std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};

fn main() {
    // 线性队列: 向量
    // 初始化方法1
    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    v1.push(3);

    assert_eq!(v1, [1, 2, 3]);
    assert_eq!(v1[1], 2);

    // 初始化方法2
    let mut v2 = vec![0; 10];
    // 初始化方法3
    let mut v3 = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);
    
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4'
    // 运行时错误
    // v3[4];

    // 双端队列
    let mut buf = VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);
    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(1), Some(&1));

    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    assert_eq!(buf.get(2), Some(&3));
    assert_eq!(buf.get(3), Some(&4));
    assert_eq!(buf.get(4), Some(&5));

    // 双向链表
    let mut list1 = LinkedList::new();
    list1.push_back('a');

    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');
    // append 用于连接 2 个链表
    // 操作后链表 2 为空
    list1.append(&mut list2);
    println!("{:?}", list1);
    println!("list2: {:?}", list2);
    
    list1.pop_front();
    println!("{:?}", list1);

    list1.push_front('e');
    println!("{:?}", list1);

    list2.push_front('f');
    println!("{:?}", list2);

    // k-v 映射表
    let mut hmap = HashMap::new();
    let mut bmap = BTreeMap::new();

    hmap.insert(3, "c");
    hmap.insert(1, "a");
    hmap.insert(2, "b");
    hmap.insert(5, "e");
    hmap.insert(4, "d");

    bmap.insert(3, "c");
    bmap.insert(2, "b");
    bmap.insert(1, "a");
    bmap.insert(5, "e");
    bmap.insert(4, "d");

    // 每次打印顺序不一致, HashMap 无序
    println!("{:?}", hmap);
    println!("{:?}", bmap);

    // 集合: HashSet 和 TreeSet
    let mut hbooks = HashSet::new();
    let mut bbooks = BTreeSet::new();

    hbooks.insert("A Song of Ice and Fire");
    hbooks.insert("The Emerald City");
    hbooks.insert("The Odyssey");

    if !hbooks.contains("The Emerald City") {
        println!("We have {} books, but The Emerald City ain't one.",
                 hbooks.len()
        );
    }

    // 无序
    println!("{:?}", hbooks);

    bbooks.insert("A Song of Ice and Fire");
    bbooks.insert("The Emerald City");
    bbooks.insert("The Odyssey");
    // 永远有序
    println!("{:?}", bbooks);

    // 优先队列
    let mut heap = BinaryHeap::new();
    // peek 取出最大值
    assert_eq!(heap.peek(), None);

    let arr = [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45];

    for &i in arr.iter() {
        heap.push(i);
    }

    assert_eq!(heap.peek(), Some(&93));
    println!("{:?}", heap);
}
