fn move_coords(x: (i32, i32)) -> (i32, i32) {
    (x.0 + 1, x.1 + 1)
}

#[derive(Debug, PartialEq)]
struct People {
    name: &'static str,
    gender: u32
}

impl People {
    // new 方法没有 &self, 调用需要 ::
    fn new(name: &'static str, gender: u32) -> Self {
        return People {
            name,
            gender
        }
    }

    fn name(&self) {
        println!("name: {:?}", self.name);
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }


    fn gender(&self){
        let gender = if (self.gender == 1) { "boy" } else { "girl" };
        println!("gender: {:?}", gender);
    }
}

// 元组结构体
// 后面需要加分号
struct Color(i32, i32, i32);

// New Type 模式: 相当于自定义类型, 更加灵活
struct Integer(u32);

// 类型别名, 但其本质还是 i32
type Int = i32;

// 单元结构体, 没有任何字段
struct Empty;

// 无参数枚举体
enum Number {
    Zero,
    One,
    Two
}

// 类 C 枚举体
enum CColor {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// 带参数枚举体
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 枚举体应用示例
enum IOption {
    Some(i32),
    None,
}


fn main() {
    // 元组
    let tuple: (&'static str, i32, char) = ("hello", 5, 'c');
    // 索引取值
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
    
    let coords = (0, 1);
    let result = move_coords(coords);
    assert_eq!(result, (1, 2));

    // 解构
    let (x, y) = move_coords(coords);
    assert_eq!(x, 1);
    assert_eq!(y, 2);

    // 结构体 - 具名结构体
    // new 方法类似构造函数, 但实际上没有构造函数
    let alex = People::new("Alex", 1);
    alex.name();
    alex.gender();
    assert_eq!(alex, People { name: "Alex", gender: 1});
    
    let mut alice = People::new("Alice", 0);
    alice.name();
    alice.gender();
    assert_eq!(alice, People { name: "Alice", gender: 0 });
    alice.set_name("Rose");
    alice.name();
    assert_eq!(alice, People { name: "Rose", gender: 0 });

    // 元组结构体
    let color = Color(0, 1, 2);
    assert_eq!(color.0, 0);
    assert_eq!(color.1, 1);
    assert_eq!(color.2, 2);

    // 相当于把 u32 包装成了新的 Integer 类型.
    let int = Integer(10);
    assert_eq!(int.0, 10);
    let int: Int = 10;
    assert_eq!(int, 10);

    // 单元结构体
    let x = Empty;
    // 在 Debug 模式下指针地址不同, Release下 相同
    println!("{:p}", &x);
    let y = x;
    println!("{:p}", &y);
    let z = Empty;
    println!("{:p}", &z);
    // 也是单元结构体
    assert_eq!((..), std::ops::RangeFull);

    // 无参数枚举体
    let a = Number::One;

    match a {
        Number::Zero => println!("0"),
        Number::One => println!("1"),
        Number::Two => println!("2"),
    }

    // 类 C 枚举体
    println!("roses are #{:06x}", CColor::Red as i32);
    println!("violets are #{:06x}", CColor::Blue as i32);

    // 带参数枚举体
    // 等号拆开, 左侧实际是函数指针类型
    let x: fn(u8, u8, u8, u8) -> IpAddr = IpAddr::V4;
    let y: fn(String) -> IpAddr = IpAddr::V6;
    let home = IpAddr::V4(127, 0, 0, 1);

    let s = Some(42);
    // 若在不确定的情况下使用 unwrap, 可能会导致运行时错误
    let num = s.unwrap();

    match s {
        Some(n) => println!("num is: {}", n),
        None => (),
    };

    // Option<T>
    let s: &Option<String> = &Some("hello".to_string());
    // Rust 2015
    match s {
        // 解构
        // ref 也是一种模式匹配, 为了解构 `&Some(ref s)` 中 s 的引用
        // 避免其中的 s 被转移所有权
        &Some(ref s) => println!("s is: {}", s),
        _ => ()
    }

    // Rust 2018
    match s {
        Some(s) => println!("s is: {}", s),
        _ => (),
    };
    
}
