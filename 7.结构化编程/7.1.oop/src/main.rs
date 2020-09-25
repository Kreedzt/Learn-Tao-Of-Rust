// 成员字段为简单原始数据类型的结构体示例
#[derive(Debug, Copy, Clone)]
struct Book<'a> {
    name: &'a str,
    isbn: i32,
    version: i32
}

// 成员字段为移动语义的情况
// error[E0204]: the trait `Copy` may not be implemented for this type
// #[derive(Debug, Copy, Clone)]
#[derive(Debug, Clone)]
struct BookMv {
    name: String,
    isbn: i32,
    version: i32
}

fn main() {
    // 成员字段为简单原始数据类型的结构体示例
    let book = Book {
        name: "Rust 编程之道",
        isbn: 20181212,
        version: 1
    };
    let book2 = Book {
        version: 2,
        // 因为结构体成员均为 复制语义类型,
        // 使用结构体更新语法 `..` 时, 所有权并未被转移
        // 这说明符合类型结构体已经通过派生属性实现了 Copy
        ..book
    };
    // 所以输出 book 时, 可以正常编译
    println!("{:?}", book);
    println!("{:?}", book2);

    // 成员字段为移动语义的情况
    let book = BookMv {
        name: "Rust 编程之道".to_string(),
        isbn: 20171111,
        version: 1
    };

    let book2 = BookMv {
        version: 2,
        // 此处更新语法啊会 转移字段的所有权
        // `name` 是 `String`, 为移动语义, 会转移所有权
        ..book
    };

    // error[E0382]: borrow of partially moved value: `book`
    // println!("{:?}", book);
    println!("{:?}", book2);
}
