// 启用特性, 使得 `nth` 示例编译通过
#![feature(try_trait)]

use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::num;
use std::num::ParseIntError;
use std::option::NoneError;
use std::process;

// 自定义错误类型 CliError
#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
    // 适配 `nth` 产生的 error
    NoneError(NoneError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
            // 适配 `nth`
            CliError::NoneError(ref err) => write!(f, "Command args error: {:?}", err),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Io(ref err) => err.description(),
            CliError::Parse(ref err) => Error::description(err),
            // 适配 `nth`
            CliError::NoneError(ref err) => "NoneError"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            CliError::Io(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
            // 适配 `nth`
            _ => None
        }
    }
}

type ParseErrorResult<i32> = Result<i32, CliError>;


// `Option<T>` 使用示例
fn get_shortest(names: Vec<&str>) -> Option<&str> {
    if names.len() > 0 {
        let mut shortest = names[0];
        for name in names.iter() {
            if name.len() < shortest.len() {
                shortest = *name;
            }
        }

        Some(shortest)
    } else {
        None
    }
}


// 使用 `match`
// fn show_shortest(names: Vec<&str>) -> &str {
//     match get_shortest(names) {
//         Some(shortest) => shortest,
//         None => "Not Found",
//     }
// }


// 使用 `unwrap`
fn show_shortest(names: Vec<&str>) -> &str {
    // 此时 `None` 会引发线程恐慌
    // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:28:25
    // get_shortest(names).unwrap()

    get_shortest(names).unwrap_or("Not Found")
    // get_shortest(names).unwrap_or_else(|| "Not Found")

    // 发生线程恐慌
    // thread 'main' panicked at 'Not Found', src/main.rs:32:25
    // get_shortest(names).expect("Not Found")
}


// 使用 `match` 匹配来操作 `Option<T>`
// fn get_shortest_length(names: Vec<&str>) -> Option<usize> {
//     match get_shortest(names) {
//         Some(shortest) => Some(shortest.len()),
//         None => None
//     }
// }


// 使用 `map` 来操作 `Option<T>`
fn get_shortest_length(names: Vec<&str>) -> Option<usize> {
    get_shortest(names).map(|name| name.len())
}


// `map` 和 `and_then` 共用示例
fn double(value: f64) -> f64 { value * 2. }

fn square(value: f64) -> f64 { value.powi(2 as i32) }

fn inverse(value: f64) -> f64 { value * -1. }

fn log(value: f64) -> Option<f64> {
    match value.log2() {
        x if x.is_normal() => Some(x),
        _ => None,
    }
}

fn sqrt(value: f64) -> Option<f64> {
    match value.sqrt() {
        x if x.is_normal() => Some(x),
        _ => None,
    }
}


// 使用 `type` 关键字定义类型别名来简化函数签名
type ParseResult<T> = Result<T, ParseIntError>;


// 解析字符串为数字错误处理示例
// fn square_parse(number_str: &str) -> Result<i32, ParseIntError> {
fn square_parse(number_str: &str) -> ParseResult<i32> {
    number_str.parse::<i32>().map(|n| n.pow(2))
}

type ParseRunResult<i32> = Result<i32, Box<Error>>;


// 重构代码, 将处理文件代码独立到 run 函数中
fn run(filename: &str) -> ParseRunResult<i32> {
    File::open(filename)
        .map_err(|e| e.into())
        .and_then(|mut f| {
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .map_err(|e| e.into())
                .map(|_| contents)
        })
        .and_then(|contents| {
            let mut sum = 0;
            for c in contents.lines() {
                match c.parse::<i32>() {
                    Ok(n) => {
                        sum += n;
                    }
                    Err(err) => {
                        let err: Box<Error> = err.into();
                        println!(
                            "error info: {}, cause: {:?}",
                            err.description(),
                            err.cause()
                        );
                    }
                }
            }
            Ok(sum)
        })
}


// 二次重构代码, 使用 `CliError`
fn run2(filename: &str) -> ParseErrorResult<i32> {
    File::open(filename)
        .map_err(CliError::Io)
        .and_then(|mut f| {
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .map_err(CliError::Io)
                .map(|_| contents)
        })
        .and_then(|contents| {
            let mut sum = 0;
            for c in contents.lines() {
                match c.parse::<i32>() {
                    Ok(n) => {
                        sum += n;
                    }
                    Err(err) => {
                        let err = CliError::Parse(err);
                        println!(
                            "error info: {}, cause: {:?}",
                            err.description(),
                            err.cause()
                        );
                    }
                }
            }
            Ok(sum)
        })
}


// 使用 try! 宏重构函数
impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError { CliError::Io(err) }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError { CliError::Parse(err) }
}


// `try!` 宏已被弃用, 当前无法编译通过
// fn run3(filename: &str) -> ParseErrorResult<i32> {
//     let mut file = try!(File::open(filename));
//     let mut contents = String::new();
//     try!(file.read_to_string(&mut contents));

//     let mut sum = 0;
//     for c in contents.lines() {
//         let n: i32 = try!(c.parse::<i32>());
//         sum += n;
//     }

//     Ok(sum)
// }


// 四次重构代码, 使用 `?` 操作符替代 `try!` 宏
fn run3(filename: &str) -> ParseErrorResult<i32> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;
    for c in contents.lines() {
        let n: i32 = c.parse::<i32>()?;
        sum += n;
    }

    Ok(sum)
}


// 适配 `nth` 产生的 `Option<String>`
impl From<NoneError> for CliError {
    fn from(err: NoneError) -> CliError { CliError::NoneError(err) }
}

// 针对 `nth` 使用 `Option` 参数
fn run4(filename: Option<String>) -> ParseErrorResult<i32> {
    let mut file = File::open(filename?)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;
    for c in contents.lines() {
        let n: i32 = c.parse::<i32>()?;
        sum += n;
    }

    Ok(sum)
}

fn main() -> Result<(), i32> {
    // assert_eq!(show_shortest(vec!["Uku", "Felipe"]), "Uku");
    // assert_eq!(show_shortest(Vec::new()), "Not Found");

    assert_eq!(get_shortest_length(vec!["Uku", "Felipe"]), Some(3));
    assert_eq!(get_shortest_length(Vec::new()), None);


    // `map` 和 `and_then` 共用示例
    let number: f64 = 20.;
    let result = Option::from(number)
        .map(inverse)
        .map(double)
        .map(inverse)
        .and_then(log)
        .map(square)
        .and_then(sqrt);

    match result {
        Some(x) => println!("Result was {}.", x),
        None => println!("This failed."),
    }


    // 使用 `parse` 方法将字符串解析为数组示例
    let n = "1";
    assert_eq!(n.parse::<i32>(), Ok(1));

    let n = "a";
    // 标准库内置的错误类型, 专门用于标识解析处理失败的错误, 此处的是指无效的数字
    // Err(ParseIntError { kind: InvalidDigit })
    println!("{:?}", n.parse::<i32>());


    // 解析字符串为数字错误处理示例
    match square_parse("10") {
        Ok(n) => assert_eq!(n, 100),
        Err(err) => println!("Error: {:?}", err),
    }


    // 从文件中读写数据并计算其和
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // let filename = &args[1];
    // let mut f = File::open(filename).unwrap();
    // let mut contents = String::new();
    // f.read_to_string(&mut contents).unwrap();


    // let mut sum = 0;
    // for c in contents.lines() {
    //     // 文字直接抛错误
    //     // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }', src/main.rs:141:34
    //     let n = c.parse::<i32>().unwrap();
    //     sum += n;
    // }
    // println!("total sum: {:?}", sum);


    // 重构代码, 将处理文件代码独立到 run 函数中
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // 无法访问会直接 panic
    // let filename = &args[1];
    // println!("In file {}", filename);

    // match run(filename) {
    //     Ok(n) => {
    //         println!("{:?}", n);
    //     }
    //     Err(e) => {
    //         println!("main error: {}", e);
    //         process::exit(1);
    //     }
    // }


    // 二次重构代码
    // match run2(filename) {
    //     Ok(n) => {
    //         println!("{:?}", n);
    //     }
    //     Err(e) => {
    //         println!("main error: {}", e);
    //         process::exit(1);
    //     }
    // }


    // 三次重构代码, `try!` 宏已被弃用
    // match run3(filename) {
    //     Ok(n) => {
    //         println!("{:?}", n);
    //     }
    //     Err(e) => {
    //         println!("main error: {}", e);
    //         process::exit(1);
    //     }
    // }


    // 四次重构代码, `?` 操作符替代 `try!` 宏
    // match run3(filename) {
    //     Ok(n) => {
    //         println!("{:?}", n);
    //     }
    //     Err(e) => {
    //         println!("main error: {}", e);
    //         process::exit(1);
    //     }
    // }


    // 使用 `env::args` 的 `nth` 方法解决 `panic` 问题
    let filename = env::args().nth(1);
    // match run4(filename) {
    //     Ok(n) => {
    //         println!("{:?}", n);
    //     }
    //     Err(e) => {
    //         // main error: Command args error: NoneError
    //         println!("main error: {}", e);
    //         process::exit(1);
    //     }
    // }


    // `main` 函数返回 `Result`
    match run4(filename) {
        Ok(n) => {
            println!("{:?}", n);
            return Ok(());
        },
        Err(e) => {
            return Err(1);
        }
    }
}
