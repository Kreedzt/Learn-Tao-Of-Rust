use std::ascii::AsciiExt;
use std::char;
use std::fmt::{self, Display, Formatter};
use std::num::ParseIntError;
use std::str;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 为自定义结构体实现 `FromStr`
impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .trim_matches(|p| p == '{' || p == '}')
            .split(",")
            .collect::<Vec<&str>>();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;
        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

// 对自定义类型 `format!` 格式化为字符串
struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

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

    // 使用 `push` 和 `push_str` 方法示例
    let mut hello = String::from("Hello, ");
    hello.push('R');
    hello.push_str("ust!");
    assert_eq!("Hello, Rust!", hello);

    // 使用 `Extend` 迭代器追加字符串
    let mut message = String::from("hello");
    message.extend([',', 'r', 'u'].iter());
    // `chars` 返回 `Chars` 迭代器
    message.extend("st ".chars());
    // `split_whiteSpace` 返回 `SplitWhitespace` 迭代器
    message.extend("w o r l d".split_whitespace());
    assert_eq!("hello,rust world", &message);

    // 使用 `insert` 和 `insert_str` 方法插入字符串
    let mut s = String::with_capacity(3);
    // 参数为插入的位置和字符
    s.insert(0, 'f');
    s.insert(1, 'o');
    s.insert(2, 'o');
    // 参数为插入的位置和字符串切片
    s.insert_str(0, "bar");
    assert_eq!("barfoo", s);

    // 使用 `"+"` 和 `"+="` 连接字符串
    let left = "the tao".to_string();
    let mut right = "Rust".to_string();
    // 操作符右边的字符串必须为切片类型, `String` 实现了 Deref trait
    // 自动解引用为 str
    assert_eq!(left + " of " + &right, "the tao of Rust");
    right += "!";
    assert_eq!(right, "Rust!");

    // 尝试使用索引来操作字符串
    let s = String::from("fooαbar");
    // 将字符串转为 `Vec<u8>` 序列
    let mut result = s.into_bytes();
    (0..result.len()).for_each(|i| {
        if i % 2 == 0 {
            // 只针对 ascii 字符, 无法针对多字节字符
            result[i] = result[i].to_ascii_lowercase();
        } else {
            result[i] = result[i].to_ascii_uppercase();
        }
    });
    // 将 `Vec<u8>` 转换为 `Result<String, FromUtf8Error>`
    assert_eq!("fOoαBaR", String::from_utf8(result).unwrap());

    // 按字符迭代来处理字符串
    let s = String::from("fooαbar");
    let s: String = s
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect();
    assert_eq!("fOoΑbAr", s);

    // 删除字符串示例
    let mut s = String::from("hαllo");
    // 删除字符串中某个位置的字符
    // 注意: `remove` 也是按字节处理字符串的, 如果给定的索引位置不是合法的字符边界, 那么线程就会崩溃
    s.remove(3);
    assert_eq!("hαlo", s);
    // 将结尾字符依次弹出, 并返回该字符
    assert_eq!(Some('o'), s.pop());
    assert_eq!(Some('l'), s.pop());
    assert_eq!(Some('α'), s.pop());
    assert_eq!("h", s);

    let mut s = String::from("hαllo");
    // 接受索引位置为参数, 将此索引位置开始到结尾的字符全部移除
    // 该方法也是按字节操作的, 注意线程崩溃问题
    s.truncate(3);
    assert_eq!("hα", s);
    // 作用同 `truncate` 传 0
    s.clear();
    assert_eq!(s, "");

    let mut s = String::from("α is alpha, βis beta");
    let beta_offset = s.find('β').unwrap_or(s.len());
    // 移除指定范围的字符
    let t: String = s.drain(..beta_offset).collect();
    assert_eq!(t, "α is alpha, ");
    assert_eq!(s, "βis beta");
    s.drain(..);
    assert_eq!(s, "");

    // 存在性判断
    let bananas = "bananas";
    // char
    assert!(bananas.contains('a'));
    // &str
    assert!(bananas.contains("an"));
    // fn pointer
    assert!(bananas.contains(char::is_lowercase));
    assert!(bananas.starts_with('b'));
    assert!(!bananas.ends_with("nana"));

    // 使用 `find` 方法查找字符位置
    let s = "Löwe 老虎 Léopard";
    assert_eq!(s.find('w'), Some(3));
    assert_eq!(s.find('老'), Some(6));
    assert_eq!(s.find('虎'), Some(9));
    assert_eq!(s.find("é"), Some(14));
    assert_eq!(s.find("Léopard"), Some(13));
    // 仅仅查询方式不一样, 结果一样, 都是正序索引
    assert_eq!(s.rfind('L'), Some(13));
    assert_eq!(s.find(char::is_whitespace), Some(5));
    assert_eq!(s.find(char::is_lowercase), Some(1));

    // `split` 系列方法使用示例
    let s = "Löwe 虎 Léopard";
    let v = s
        // 通过码位范围锁定中文字符
        // U+4E00 ~ U+9FA5
        .split(|c| (c as u32) >= (0x4E00 as u32) && (c as u32) <= (0x9FA5 as u32))
        .collect::<Vec<&str>>();
    assert_eq!(v, ["Löwe ", " Léopard"]);

    let v = "abc1defXghi"
        .split(|c| c == '1' || c == 'X')
        .collect::<Vec<&str>>();
    assert_eq!(v, ["abc", "def", "ghi"]);

    let v = "Mary had a little lambda"
        // n 代表分割的数组长度
        .splitn(3, ' ')
        .collect::<Vec<&str>>();
    assert_eq!(v, ["Mary", "had", "a little lambda"]);

    let v = "A.B.".split(".").collect::<Vec<&str>>();
    assert_eq!(v, ["A", "B", ""]);

    let v = "A.B.".split_terminator('.').collect::<Vec<&str>>();
    assert_eq!(v, ["A", "B"]);

    let v = "A..B..".split(".").collect::<Vec<&str>>();
    assert_eq!(v, ["A", "", "B", "", ""]);

    let v = "A..B..".split_terminator(".").collect::<Vec<&str>>();
    assert_eq!(v, ["A", "", "B", ""]);

    // `matches` 系列方法使用示例
    let v = "abcXXXabcYYYabc".matches("abc").collect::<Vec<&str>>();
    assert_eq!(v, ["abc", "abc", "abc"]);

    let v = "1abc2abc3"
        // 因反向匹配, 所以得到的迭代器也是反向的
        .rmatches(char::is_numeric)
        .collect::<Vec<&str>>();
    assert_eq!(v, ["3", "2", "1"]);

    let v = "abcXXXabcYYYabc"
        // 返回元组数组
        .match_indices("abc")
        .collect::<Vec<_>>();
    assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);

    let v = "abcXXXabcYYYabc".rmatch_indices("abc").collect::<Vec<_>>();
    assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);

    // `trim` 系列方法使用示例
    let s = " Hello\tworld\t";
    assert_eq!("Hello\tworld", s.trim());
    assert_eq!("Hello\tworld\t", s.trim_left());
    assert_eq!(" Hello\tworld", s.trim_right());

    // `trim_matches` 系列方法使用示例
    // 警告: 此处示例代码有误, 已改动
    assert_eq!("Hello\tworld\t".trim_matches('\t'), "Hello\tworld");
    assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
    assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar");

    let x: &[char] = &['1', '2'];
    assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
    assert_eq!(
        "1foo1barXX".trim_matches(|c| c == '1' || c == 'X'),
        "foo1bar"
    );
    assert_eq!("11foo1bar11".trim_left_matches('1'), "foo1bar11");
    assert_eq!(
        "123foo1bar123".trim_left_matches(char::is_numeric),
        "foo1bar123"
    );

    let x: &[char] = &['1', '2'];
    assert_eq!("12foo1bar12".trim_left_matches(x), "foo1bar12");
    // 警告:　有改动
    assert_eq!("1fooX".trim_left_matches(|c| c == '1' || c == 'X'), "fooX");

    // `replace` 系列方法使用示例
    let s = "Hello\tworld\t";
    assert_eq!("Hello world ", s.replace("\t", " "));
    assert_eq!("Hello world", s.replace("\t", " ").trim());

    let s = "this is old old 123";
    assert_eq!("this is new new 123", s.replace("old", "new"));
    assert_eq!("this is new old 123", s.replacen("old", "new", 1));
    assert_eq!("this is ald ald 123", s.replacen('o', "a", 3));
    assert_eq!(
        "this is old old new23",
        s.replacen(char::is_numeric, "new", 1)
    );

    // `parse` 方法使用示例
    // 该方法是为泛型方法, 需要指定类型
    let four: u32 = "4".parse().unwrap();
    assert_eq!(4, four);
    let four = "4".parse::<u32>();
    assert_eq!(Ok(4), four);

    // 为自定义结构体实现 `FromStr`
    let p = Point::from_str("{1,2}");
    assert_eq!(p.unwrap(), Point { x: 1, y: 2 });
    // Err(ParseIntError { kind: InvalidDigit })
    let p = Point::from_str("{3,u}");
    println!("{:?}", p);

    // 使用 `format!` 根据字符串生成字符串
    let s: String = format!("{}Rust", "Hello");
    assert_eq!(s, "HelloRust");

    // :5 表示填充字符串宽度
    assert_eq!(format!("{:5}", "HelloRust"), "HelloRust");
    // :5 表示填充字符串宽度, .3 表示截取字符长度
    assert_eq!(format!("{:5.3}", "HelloRust"), "Hel  ");
    assert_eq!(format!("{:10}", "HelloRust"), "HelloRust ");
    // 左对齐
    assert_eq!(format!("{:<12}", "HelloRust"), "HelloRust   ");
    // 右对齐
    assert_eq!(format!("{:>12}", "HelloRust"), "   HelloRust");
    // 中间对齐
    assert_eq!(format!("{:^12}", "HelloRust"), " HelloRust  ");
    // 中间对齐, 截取 5, 12宽度
    assert_eq!(format!("{:^12.5}", "HelloRust"), "   Hello    ");
    // 用 `=` 填充
    assert_eq!(format!("{:=^12.5}", "HelloRust"), "===Hello====");
    assert_eq!(format!("{:*^12.5}", "HelloRust"), "***Hello****");
    assert_eq!(format!("{:5}", "th\u{e9}"), "thé  ");

    // 针对整数使用 `format!` 格式化为字符串
    // 显示正负符号
    assert_eq!(format!("{:+}", 1234), "+1234");
    // x: 十六进制
    assert_eq!(format!("{:+x}", 1234), "+4d2");
    // #: 显示进制前缀
    assert_eq!(format!("{:+#x}", 1234), "+0x4d2");
    // b: 二进制
    assert_eq!(format!("{:b}", 1234), "10011010010");
    // 二进制 + 前缀
    assert_eq!(format!("{:#b}", 1234), "0b10011010010");
    assert_eq!(format!("{:#20b}", 1234), "       0b10011010010");
    assert_eq!(format!("{:<#20b}", 1234), "0b10011010010       ");
    assert_eq!(format!("{:^#20b}", 1234), "   0b10011010010    ");
    assert_eq!(format!("{:>+#15x}", 1234), "         +0x4d2");
    // 以 0 填充
    assert_eq!(format!("{:>+#015x}", 1234), "+0x0000000004d2");

    // 针对浮点数使用 `format!` 格式化字符串
    assert_eq!(format!("{:.4}", 1234.5678), "1234.5678");
    assert_eq!(format!("{:.2}", 1234.5618), "1234.56");
    assert_eq!(format!("{:.2}", 1234.5678), "1234.57");
    assert_eq!(format!("{:<10.4}", 1234.5678), "1234.5678 ");
    assert_eq!(format!("{:^12.2}", 1234.5618), "  1234.56   ");
    assert_eq!(format!("{:0^12.2}", 1234.5678), "001234.57000");
    assert_eq!(format!("{:e}", 1234.5678), "1.2345678e3");

    // 对自定义类型 `format!` 格式化为字符串
    let city = City { name: "Beijing", lat: 39.90469, lon: -116.40717};
    assert_eq!(format!("{}", city), "Beijing: 39.905°N 116.407°W");
    println!("{}", city);
}
