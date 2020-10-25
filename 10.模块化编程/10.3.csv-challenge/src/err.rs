// 定义统一的错误类型
use std::io;

#[derive(Debug)]
pub enum Error {
    // I/O 错误
    Io(io::Error),
    // 逻辑错误
    Program(&'static str),
}

impl From<io::Error> for Error{
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Error {
        Error::Program(e)
    }
}
