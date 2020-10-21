// Rust 2015 版本写法, 2018 以下皆可省略
// extern crate linked_list;
// extern crate regex;
// #[macro_use]
// extern crate lazy_static;

use regex::Regex;
// Rust 2018: https://doc.rust-lang.org/edition-guide/rust-2018/macros/macro-changes.html
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?x)
(?P<year>\d{4})- # the year
(?P<month>\d{2})- # the month
(?P<day>\d{2}) # the day
").unwrap();
    static ref EMAIL_RE: Regex = Regex::new(r"(?x)
^\w+@(?:gmail|163|qq)\.(?:com|cn|com\.cn|net)$
").unwrap();
}

const TO_SEARCH: &'static str = "
On 2017-12-31, happy. On 2018-01-01, New Year.
";

fn regex_date(text: &str) -> regex::Captures { RE.captures(text).unwrap() }

fn regex_email(text: &str) -> bool { EMAIL_RE.is_match(text) }

fn main() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

    // 进行匹配和迭代
    for caps in re.captures_iter(TO_SEARCH) {
        println!(
            "year: {}, month: {}, day: {}",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str()
        );
    }


    // 使用命名捕获的示例
    let re = Regex::new(
        // 指定正则表达式标记 x
        r"(?x)
(?P<year>\d{4}) # this year
-
(?P<month>\d{2}) # this month
-
(?P<day>\d{2}) # this day
",
    )
    .unwrap();

    // 使用该方法可以获取匹配的捕获变量, 并保存到一个 `HashMap` 中
    // 以命名变量作为键, 匹配的字符串作为值
    let caps = re.captures("2018-01-01").unwrap();
    assert_eq!("2018", &caps["year"]);
    assert_eq!("01", &caps["month"]);
    assert_eq!("01", &caps["day"]);

    // 按指定的格式替换匹配的字符串. 以 "$" 符号和命名捕获变量组合而成
    let after = re.replace_all("2018-01-01", "$month/$day/$year");
    assert_eq!(after, "01/01/2018");


    // 使用 lazy_static
    let caps = regex_date("2018-01-01");
    assert_eq!("2018", &caps["year"]);
    assert_eq!("01", &caps["month"]);
    assert_eq!("01", &caps["day"]);

    let after = RE.replace_all("2018-01-01", "$month/$day/$year");

    assert_eq!(after, "01/01/2018");
    assert!(regex_email("alex@gmail.com"), true);
    assert_eq!(regex_email("alex@gmail.cn.com"), false);
}
