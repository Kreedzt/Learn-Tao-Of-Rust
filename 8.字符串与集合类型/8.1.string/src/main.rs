use std::str;

fn main() {
    // 字符串编码示例
    // 将 UTF-8 字节序列转换为字符串 `"道"`
    // 使用 `u8` 表示字节类型
    let tao = str::from_utf8(&[0xE9u8,0x81u8,0x93u8]).unwrap();
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
}
