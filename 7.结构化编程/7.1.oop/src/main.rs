// 成员字段为简单原始数据类型的结构体示例
#[derive(Debug, Copy, Clone)]
struct Book<'a> {
    name: &'a str,
    isbn: i32,
    version: i32,
}

// 成员字段为移动语义的情况
// error[E0204]: the trait `Copy` may not be implemented for this type
// #[derive(Debug, Copy, Clone)]
#[derive(Debug, Clone)]
struct BookMv {
    name: String,
    isbn: i32,
    version: i32,
}

// 定义元组结构体并为其实现 `Drop`
// 此处使用 `Newtype` 模式只是为了提升语义, 增强可读性, 方便后面示例代码的展示
struct PrintDrop(&'static str);
impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

// 结构体和枚举体的析构熟悉怒
struct Foo {
    x: PrintDrop,
    y: PrintDrop,
    z: PrintDrop,
}

enum E {
    Foo(PrintDrop, PrintDrop),
}

fn main() {
    // 成员字段为简单原始数据类型的结构体示例
    let book = Book {
        name: "Rust 编程之道",
        isbn: 20181212,
        version: 1,
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
        version: 1,
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

    // 本地变量的析构
    // 先析构的是 `y`, 改变放置顺序有可能会导致悬垂指针
    // 在编写 Rust 代码时, 有时只要修改一下变量的声明顺序, 本来无法编译的代码就可以正常编译通过了
    // let x = PrintDrop("x");
    // let y = PrintDrop("y");

    // 元组的析构
    // 元组内部是按元素的出现顺序依次进行析构的
    // let tup1 = (PrintDrop("a"), PrintDrop("b"), PrintDrop("c"));
    // let tup2 = (PrintDrop("x"), PrintDrop("y"), PrintDrop("z"));

    // 将 tup2 中的最后一个元素修改为 `panice!()`
    // let tup1 = (PrintDrop("a"), PrintDrop("b"), PrintDrop("c"));
    // 线程的崩溃触发了 `tup2` 的提前析构, 顺序正好和局部变量的析构顺序一致:
    // 先声明的后析构
    // 依次会触发其他变量的析构
    // let tup2 = (PrintDrop("x"), PrintDrop("y"), panic!());

    // 枚举体和结构体
    // let e = E::Foo(PrintDrop("a"), PrintDrop("b"));
    // 内部析构顺序按排列顺序来析构
    // let f = Foo {
    //     x: PrintDrop("x"),
    //     y: PrintDrop("y"),
    //     z: PrintDrop("z")
    //     // 与元组表现一致
    //     // z: panic!()
    // };

    // 闭包捕获变量
    // 与捕获变量的 *声明顺序无关*, 与闭包内变量的排列顺序一致
    // let x = PrintDrop("x");
    // let z = PrintDrop("z");
    // let y = PrintDrop("y");
    // let closure = move || {
    //     y;
    //     z;
    //     x;
    // };

    // 闭包捕获变量析构顺序变化的特殊情况.
    let y = PrintDrop("y");
    let x = PrintDrop("x");
    let z = PrintDrop("z");
    let closure = move || {
        {
            // 使用了一个内部作用域来引用变量 `z`
            // z 在 move 到闭包之前先被借用了, 所以需要等待其离开作用域归还后, 才能被 move 到闭包中
            let z_ref = &z;
        }
        x;
        y;
        z;
    };
}
