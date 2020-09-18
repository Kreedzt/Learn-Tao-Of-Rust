fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() % 2 == 0 {
        x
    } else {
        y
    }
}

fn capitalize(data: &mut[char]) {
    
}

// NLL 示例之二
struct List<T> {
    value: T,
    next: Option<Box<List<T>>>
}

// NLL 示例之二
fn to_refs<T>(mut list: &mut List<T>) -> Vec<&mut T> {
    let mut result = vec![];
    loop {
        // 在循环中使用了 `&mut list.value`
        // 早期借用检查器可能会认为这个可变借用是多次借用, 从而报错
        // NLL 解决了无限循环中借用检查的问题
        result.push(&mut list.value);
        if let Some(n) = list.next.as_mut() {
            list = n;
        } else {
            return result;
        }
    }
}

fn main() {
    // Rust 借用检查机制粒度问题,
    // Rust 2018 可以编译通过
    let x = String::from("hello");
    let z;
    let y = String::from("world");
    z = foo(&x, &y);
    println!("{:?}", z);

    // NLL 示例之一
    // Rust 2018 可以编译通过
    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..];
    // 在执行了 `capitalize()` 函数之后, 理论上就没有需要使用可变借用 `&mut data`
    // 但是早期借用检查是基于词法作用域的. `&mut data` 的生命周期会被认为是独占了函数整个作用域范围的,
    // 所以当 `data` 调用 `push()` 方法并需要可变借用的时候, 违反了借用规则.
    capitalize(slice);
    data.push('d');

    // NLL 示例之三
    let mut x = vec![1];
    // error[E0499]: cannot borrow `x` as mutable more than once at a time
    x.push(x.pop().unwrap());
}
