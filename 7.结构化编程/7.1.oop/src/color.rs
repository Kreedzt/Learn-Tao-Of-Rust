use std::fmt;

struct ColoredString {
    // 输入的原始字符串
    input: String,
    // 前景色
    fgcolor: String,
    // 背景色
    bgcolor: String
}

// trait 可以实现非侵入式接口
trait Colorize {
    // 关联常量, 并定义了默认实现
    // 常量名必须大写, 且需要明确标注类型
    const FG_RED: &'static str = "31";
    const BG_YELLOW: &'static str = "43";
    // const TZ_COLOR: &'static str;

    // `self` 代表 `self:Self`, `Self` 代表实现该 trait 的类型
    // 关联函数允许开发者使用点操作符来调用函数, 同样也支持链式调用.
    fn red(self) -> ColoredString;
    fn on_yellow(self) -> ColoredString;
}

// `Default` 已在 `std::prelude::v1` 模块中被导入
impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: String::default(),
            bgcolor: String::default()
        }
    }
}

impl<'a> Colorize for ColoredString {
    // 返回 `ColoredString` 实现链式调用
    fn red(self) -> ColoredString {
        ColoredString {
            fgcolor: String::from(ColoredString::FG_RED),
            ..self
        }
    }

    fn on_yellow(self) -> ColoredString {
        ColoredString {
            bgcolor: String::from(ColoredString::BG_YELLOW),
            ..self
        }
    }
}

impl<'a> Colorize for &'a str {
    // 返回 `ColoredString` 实现链式调用
    fn red(self) -> ColoredString {
        ColoredString {
            fgcolor: String::from(ColoredString::FG_RED),
            input: String::from(self),
            ..ColoredString::default()
        }
    }

    fn on_yellow(self) -> ColoredString {
        ColoredString {
            bgcolor: String::from(ColoredString::BG_YELLOW),
            input: String::from(self),
            ..ColoredString::default()
        }
    }
}


impl ColoredString {
    fn compute_style(&self) -> String {
        let mut res = String::from("\x1B[");
        // 用于判断是否有 `bgcolor` 的设置
        let mut has_wrote = false;
        if !self.bgcolor.is_empty() {
            // 需要 `&str` 类型
            res.push_str(&self.bgcolor);
            has_wrote = true;
        }

        if !self.fgcolor.is_empty() {
            if has_wrote {
                res.push(';');
            }

            res.push_str(&self.fgcolor);
        }

        res.push('m');
        res
    }
}

// 实现 `Display`, 通过 `{}` 打印
impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 存储记录的原始文本
        let mut input = &self.input.clone();
        try!(f.write_str(&self.compute_style()));
        try!(f.write_str(input));
        try!(f.write_str("\x1B[0m"));
        Ok(())
    }
}


fn main() {
    let hi = "Hello".red().on_yellow();
    println!("{}", hi);
    let hi = "Hello".on_yellow();
    println!("{}", hi);
    let hi = "Hello".red();
    println!("{}", hi);
    let hi = "Hello".on_yellow().red();
    println!("{}", hi);
}
