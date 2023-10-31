use std::fmt::{write, Display, Formatter};

use CustomError::*;

#[derive(Debug)]
pub enum CustomError {
    PathNotFount,
    IoError,
    CreateFail(&'static str),
    ParseError(&'static str),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathNotFount => write!(f, "配置文件地址找不到"),
            IoError => write!(f, "io 读取错误"),
            CreateFail(p) => write!(f, "配置文件创建失败,文件地址：{}", p),
            ParseError(s) => write!(f, "解析错误，错误原因:{}", s),
        }
    }
}
