// 12-50 在 `src/lib.rs` 中实现 `#[derive(A)]` 过程宏
extern crate proc_macro;
use self::proc_macro::TokenStream;

#[proc_macro_derive(A)]
pub fn derive(input: TokenStream) -> TokenStream {
    // 输入转为字符串处理
    let input = input.to_string();
    // 注意: 此处加 `;` 报错, 原因未知
    assert!(input.contains("struct A"));
    r#"
      impl A {
          fn a(&self) -> String {
              format!("hello from impl A")
          }
      }
    "#
    .parse()
    .unwrap()
}


// 12-51 在 `src/lib.rs` 中继续编写自定义属性的实现代码
#[proc_macro_attribute]
pub fn attr_with_args(args: TokenStream, input: TokenStream) -> TokenStream {
    // args 为 括号内文本
    let args = args.to_string();
    let input = input.to_string();
    format!("fn foo() -> &'static str {{ {} }}", args)
        .parse()
        .unwrap()
}


// 12-53 继续编写 `hashmap!` 实现代码
#[proc_macro]
pub fn hashmap(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input = input.trim_end_matches(',');
    let input: Vec<String> = input
        .split(",")
        .map(|n| {
            let mut data = if n.contains(":") {
                n.split(":")
            } else {
                n.split(" => ")
            };

            let (key, value) = (data.next().unwrap(), data.next().unwrap());
            format!("hm.insert({}, {})", key, value)
        })
        .collect();

    let count: usize = input.len();

    let tokens = format!(
        "
        {{
            let mut hm =
                ::std::collections::HashMap::with_capacity({});
            {}
            hm
}}",
        count,
        input.iter().map(|n| format!("{};", n)).collect::<String>()
    );

    tokens.parse().unwrap()
}
