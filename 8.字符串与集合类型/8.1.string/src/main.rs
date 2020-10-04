use std::char;
use std::str;

fn main() {
    // 字符串编码示例
    // 将 UTF-8 字节序列转换为字符串 `"道"`
    // 使用 `u8` 表示字节类型
    let tao = str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
    assert_eq!("道", tao);
    // 将一个 16 进制码位转换为字符串 `"道"`
    assert_eq!("道", String::from("\u{9053}"));
    // 十六进制码位
    let unicode_x = 0x9053;
    // UTF-8 编码之后的十六进制表示
    let utf_x_hex = 0xe98193;
    // UTF-8 编码之后的二进制表示
    let utf_x_bin = 0b111010011000000110010011;
    println!("unicode_x: {:b}", unicode_x);
    println!("utf_x_hex: {:b}", utf_x_hex);
    println!("utf_x_bin: 0x{:x}", utf_x_bin);

    // 字符与标量值一一对应
    let tao = '道';
    // `as` 转为 `u32` 类型
    let tao_u32 = tao as u32;
    // 整数值
    assert_eq!(36947, tao_u32);
    // 十六进制, 对应 Unicode 标量值
    println!("U+{:x}", tao_u32);
    println!("{}", tao.escape_unicode());
    assert_eq!(char::from(65), 'A');
    assert_eq!(char::from_u32(0x9053), Some('道'));
    assert_eq!(char::from_u32(36947), Some('道'));
    // assert_eq!(char::from_u32(12901010101), None);

    // 将字符转换为字符串, 要注意字节长度
    // 因为字符串 '道' 的 UTF-8 编码占 3 字节, 所以, 如果要转换为合法字符串, 则数组长度至少为 3
    let mut b = [0; 3];
    let tao = '道';
    let tao_str = tao.encode_utf8(&mut b);
    assert_eq!("道", tao_str);
    // 通过内建的方法获取 UTF-8 编码的字节长度
    assert_eq!(3, tao.len_utf8());

    // 包含 2 个码位的字符示例
    // Rust 1.3 版本起, 开始支持多码位字符
    let e = 'é';
    println!("{}", e as u32);

    // 字符内建的常用方法示例
    assert_eq!(true, 'f'.is_digit(16));
    assert_eq!(Some(15), 'f'.to_digit(16));
    assert!('a'.is_lowercase());
    assert!(!'道'.is_lowercase());
    assert!(!'a'.is_uppercase());
    assert!('A'.is_uppercase());
    assert!(!'中'.is_uppercase());
    // 内容改动:　https://github.com/ZhangHanDong/tao-of-rust-codes/issues/240
    assert_eq!('i', 'I'.to_ascii_lowercase());
    assert_eq!('B', 'b'.to_ascii_uppercase());
    assert!(' '.is_whitespace());
    assert!('\u{A0}'.is_whitespace());
    assert!(!'越'.is_whitespace());
    assert!('a'.is_alphabetic());
    // 汉字也是字母
    assert!('京'.is_alphabetic());
    assert!(!'1'.is_alphabetic());
    // 是否字母/数字
    assert!('7'.is_alphanumeric());
    assert!('K'.is_alphanumeric());
    assert!('藏'.is_alphanumeric());
    // 内容改动:　https://github.com/ZhangHanDong/tao-of-rust-codes/issues/240
    // 判定为字母
    assert!('¾'.is_alphanumeric());
    assert!(''.is_control());
    assert!(!'q'.is_control());
    assert!('٣'.is_numeric());
    assert!('7'.is_numeric());
    assert!(!'و'.is_numeric());
    assert!(!'藏'.is_numeric());
    // 用于转义  \t, \r, \n
    println!("{}", '\r'.escape_default());


    // 组成 `String` 类型的三部分
    let mut a = String::from("fooα");
    // 堆中字节序列的地址
    println!("{:p}", a.as_ptr());
    // 字符串变量在栈上的地址
    println!("{:p}", &a);
    assert_eq!(a.len(), 5);
    // 5
    println!("capacity: {}", a.capacity());
    // 再次分配容量
    a.reserve(10);
    assert_eq!(a.capacity(), 15);

    
    // 创建字符串的各种方法示例
    let string: String = String::new();
    assert_eq!("", string);

    let string: String = String::from("hello rust");
    assert_eq!("hello rust", string);

    // 容量只是存储空间(eg: 堆)的一种刻度
    // 实际申请的堆内存空间为每个字符的字节大小 * 容量值
    let string: String = String::with_capacity(20);
    assert_eq!("", string);

    let str: &'static str = "the tao of rust";
    let string: String = str.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!("thetaoofrust", string);

    // 与后者性能相差无几
    // 利用 `&str` 切片字节序列生成新的 `String` 字符串
    let string: String = str.to_owned();
    assert_eq!("the tao of rust", string);

    // 与前者性能相差无几
    // 对 `String::from` 的包装
    let string: String = str.to_string();
    let str: &str = &string[11..15];
    assert_eq!("rust", str);

    // 使用 chars 和 bytes 方法示例
    let str = "borös";
    // 按码位迭代
    let mut chars = str.chars();
    assert_eq!(Some('b'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('r'), chars.next());
    assert_eq!(Some('ö'), chars.next());
    assert_eq!(Some('s'), chars.next());

    // 按字节迭代
    let mut bytes = str.bytes();
    // 返回字符串字节长度, 而非字符长度
    assert_eq!(6, str.len());
    assert_eq!(Some(98), bytes.next());
    assert_eq!(Some(111), bytes.next());
    assert_eq!(Some(114), bytes.next());
    assert_eq!(Some(195), bytes.next());
    assert_eq!(Some(182), bytes.next());
    assert_eq!(Some(115), bytes.next());

    // 使用 `get` 和 `get_mut` 方法示例
    let mut v = String::from("borös");
    assert_eq!(Some("b"), v.get(0..1));
    assert_eq!(Some("ö"), v.get(3..5));
    assert_eq!(Some("orös"), v.get(1..));
    assert!(v.get_mut(4..).is_none());
    // 判断是否合法的字符边界
    assert!(!v.is_char_boundary(4));
    assert!(v.get_mut(..8).is_none());
    assert!(v.get_mut(..42).is_none());

    // 使用 `split_at` 方法示例
    let s = "Per Martin-Löf";
    let (first, last) = s.split_at(12);
    assert_eq!("Per Martin-L", first);
    assert_eq!("öf", last);
    // thread 'main' panicked at 'byte index 13 is not a char boundary; it is inside 'ö' (bytes 12..14) of `Per Martin-Löf`', C:\Users\Ken Zhao\.
    // 13 为字节序列中间位置, 为非法的字符边界.
    // let (first, last) = s.split_at(13);
}
