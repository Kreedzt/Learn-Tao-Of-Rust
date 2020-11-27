#![feature(plugin_registrar, rustc_private)]
// Rust 1.50 - Nightly 下无法编译
extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

use self::rustc_plugin::Registry;
use self::syntax::ext::base::{DummyResult, ExtCtxt, MacEager, MacResult};
use self::syntax::ext::build::AstBuilder;
use self::syntax::ext::quote::rt::Span;
use self::syntax::parse::token;
use self::syntax::tokenstream::TokenStream;

static ROMAN_NUMERALS: &'static [(&'static str, usize)] = &[
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

// 参数说明:
// cx: 代码代码的上下文环境, 为 ExtCtxt 的可变引用类型
// Span: 表示代码的位置等信息
// TokenTree: 切片数组, 表示经过编译器分词器得到的代码词条树
// 返回: Box<MacResult + 'static>: 是一个 trait 对象.
// 该 trait 中定义了很多方便用于组装 AST 结构. 因为编译器插件是直接修改 AST 结构来实现语法扩展的.
fn expand_roman(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    // text 绑定, 检测第一个值是否是标识符
    let text = match args[0] {
        TokenTree::Token(_, token::Ident(s, _)) => s.to_string(),
        _ => {
            cx.span_err(sp, "argument should be a single identifier");
            return DummyResult::any(sp);
        }
    };

    let mut text = &*text;
    // 最终的阿拉伯数字
    let mut total = 0;
    while !text.is_empty() {
        match ROMAN_NUMERALS.iter().find(|&&(rn, _)| text.starts_with(rn)) {
            Some(&(rn, val)) => {
                total += val;
                text = &text[rn.len()..];
            }
            None => {
                cx.span_err(sp, "invalid Roman numeral");
                return DummyResult::any(sp);
            }
        }
    }

    MacEager::expr(cx.expr_usize(sp, total))
}


// 12-74 定义宏
#[plugin_registrar]
pub fn roman_to_digit(reg: &mut Registry) { reg.register_macro("roman_to_digit", expand_roman); }
