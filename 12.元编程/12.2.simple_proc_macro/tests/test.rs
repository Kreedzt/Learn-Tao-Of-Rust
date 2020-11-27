use simple_proc_macro::attr_with_args;
use simple_proc_macro::hashmap;

// *注意*: 使用自定义的派生属性过程宏, 需要用 ~#[macro_use]~ 将其导出
#[macro_use]
extern crate simple_proc_macro;

#[derive(A)]
struct A;

#[test]
fn test_derive_a() {
    // 实现一个 *自定义派生属性 `#[derive(A)]` *, 然后为单元结构体 A 自动实现一个实例方法 a
    // 在调用的时候输出指定的字符串
    assert_eq!("hello from impl A".to_string(), A.a());
}


// 12-50 继续编写自定义属性测试代码
#[attr_with_args("Hello, Rust!")]
fn foo() {}

#[test]
fn test_foo() {
    assert_eq!(foo(), "Hello, Rust!");
}


// 12-52 继续实现 `hashmap!` 测试用例
#[test]
fn test_hashmap() {
    let hm = hashmap! { "a": 1, "b": 2, };
    assert_eq!(hm["a"], 1);

    let hm = hashmap! { "a" => 1, "b" => 2, "c" => 3};
    assert_eq!(hm["d"], 4);
}
