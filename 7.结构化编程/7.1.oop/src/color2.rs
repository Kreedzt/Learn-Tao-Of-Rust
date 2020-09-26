use std::convert::From;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Color {
    Red,
    Yellow,
    Blue,
}

impl Color {
    fn to_fg_str(&self) -> &str {
        // 使用 match 匹配时必须匹配所有值
        // 此处的 `*self` 并不会获得所有权
        match *self {
            Color::Red => "31",
            Color::Yellow => "33",
            Color::Blue => "34",
        }
    }

    fn to_bg_str(&self) -> &str {
        match *self {
            Color::Red => "41",
            Color::Yellow => "43",
            Color::Blue => "44",
        }
    }
}

// 修改 `ColoredString` 结构体中的类型
#[derive(Clone, Debug, PartialEq, Eq)]
struct ColoredString {
    // 输入的原始字符串
    input: String,
    // 前景色
    fgcolor: Option<Color>,
    // 背景色
    bgcolor: Option<Color>,
}

// `Default` 已在 `std::prelude::v1` 模块中被导入
impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: None,
            bgcolor: None,
        }
    }
}

// 为 `Color` 实现 `From`
impl<'a> From<&'a str> for Color {
    fn from(src: &str) -> Self {
        // `parse` 方法要求目标类型必须实现 `FromStr`
        src.parse().unwrap_or(Color::Red)
    }
}

impl From<String> for Color {
    fn from(src: String) -> Self { src.parse().unwrap_or(Color::Red) }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let src = src.to_lowercase();

        match src.as_ref() {
            "red" => Ok(Color::Red),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            // 使用通配符匹配可能值之外的情况
            _ => Err(()),
        }
    }
}

trait Colorize {
    fn red(self) -> ColoredString;
    fn yellow(self) -> ColoredString;
    fn blue(self) -> ColoredString;
    fn color<S: Into<Color>>(self, color: S) -> ColoredString;
    fn on_red(self) -> ColoredString;
    fn on_yellow(self) -> ColoredString;
    fn on_blue(self) -> ColoredString;
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString;
}

impl<'a> Colorize for ColoredString {
    fn red(self) -> ColoredString { self.color(Color::Red) }
    fn yellow(self) -> ColoredString { self.color(Color::Yellow) }
    fn blue(self) -> ColoredString { self.color(Color::Blue) }
    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            fgcolor: Some(color.into()),
            ..self
        }
    }
    fn on_red(self) -> ColoredString { self.on_color(Color::Red) }
    fn on_yellow(self) -> ColoredString { self.on_color(Color::Yellow) }
    fn on_blue(self) -> ColoredString { self.on_color(Color::Blue) }
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            bgcolor: Some(color.into()),
            ..self
        }
    }
}

impl<'a> Colorize for &'a str {
    fn red(self) -> ColoredString { self.color(Color::Red) }
    fn yellow(self) -> ColoredString { self.color(Color::Yellow) }
    fn blue(self) -> ColoredString { self.color(Color::Blue) }
    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            fgcolor: Some(color.into()),
            input: String::from(self),
            ..ColoredString::default()
        }
    }

    fn on_red(self) -> ColoredString { self.on_color(Color::Red) }
    fn on_yellow(self) -> ColoredString { self.on_color(Color::Yellow) }
    fn on_blue(self) -> ColoredString { self.on_color(Color::Blue) }
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            bgcolor: Some(color.into()),
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

        if let Some(ref bgcolor) = self.bgcolor {
            if has_wrote {
                res.push(';');
            }
            res.push_str(bgcolor.to_bg_str());
            has_wrote = true;
        }

        if let Some(ref fgcolor) = self.fgcolor {
            if has_wrote {
                res.push(';');
            }

            res.push_str(fgcolor.to_fg_str());
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
    let red = "red".red();
    println!("{}", red);

    let yellow = "yellow".yellow().on_blue();
    println!("{}", yellow);

    let blue = "blue".blue();
    println!("{}", blue);

    let red = red.color("red");
    println!("{}", red);

    let yellow = "yellow".on_color("yellow");
    println!("{}", yellow);
}
