extern crate failure;
#[macro_use]
extern crate failure_derive;
use failure::{Backtrace, Context, Fail};
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "IoError")]
    // 通过 `cause` 指定标准库中内置的基础错误类型.
    Io(#[cause] std::io::Error),
    #[fail(display = "ParseError")]
    Parse(#[cause] std::num::ParseIntError),
    // 增加新的 `Error` 种类
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> { self.inner.cause() }

    fn backtrace(&self) -> Option<&Backtrace> { self.inner.backtrace() }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            // 书中代码不一致
            inner: Context::new(ErrorKind::Io(err)),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error {
            inner: Context::new(ErrorKind::Parse(err)),
        }
    }
}

type ParseResult<i32> = Result<i32, Error>;

fn run(filename: Option<String>) -> ParseResult<i32> {
    let mut file = File::open(filename.unwrap())?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;

    for c in contents.lines() {
        let n: i32 = c.parse::<i32>()?;
        sum += n;
    }

    Ok(sum)
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1);

    match run(filename) {
        Ok(n) => {
            println!("{:?}", n);
            return Ok(());
        },
        Err(e) => {
            return Err("1".to_string());
        }
    }
}
