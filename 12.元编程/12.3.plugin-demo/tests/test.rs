#![feature(plugin)]
#![plugin(plugin_demo)] // 将 plugin-demo 中定义的语法扩展导出
#[test]
fn test_plugin() {
    assert_eq!(roman_to_digit!(MMXVIII), 2018);
}
