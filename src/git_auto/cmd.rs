use std::env;
use std::env::Args;
use std::fmt::{write, Display, Error, Formatter};
use std::str::FromStr;

use self::Action::*;
use self::CmdError::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Action {
    NewFeature,
    Issues,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NewFeature => write!(f, "创建新需求"),
            Issues => write!(f, "所有的 issue 列表"),
        }
    }
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nf" | "newFeature" | "new_feature" => Ok(NewFeature),
            "iss" | "issues" => Ok(Issues),
            _ => Err(()),
        }
    }
}

pub enum CmdError {
    NotSupportCmd(&'static str),
    ValueError(&'static str),
    UnKnow,
}

impl Display for CmdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotSupportCmd(str) => write!(f, "不支持的命令{}", str),
            ValueError(str) => write!(f, "命令值错误{}", str),
            UnKnow => write!(f, "未知的错误"),
        }
    }
}

#[derive(Debug)]
pub struct Cmd<'a> {
    pub action: Action,
    pub val: Option<&'a str>,
}

impl<'a> Cmd<'a> {
    pub fn new(args: &'a Vec<String>) -> Self {
        let res = Cmd::parse(args);
        Cmd {
            action: res.0,
            val: res.1,
        }
    }
    fn parse(args: &'a Vec<String>) -> (Action, Option<&'a str>) {
        let action = args.get(1);
        let action = match action {
            Some(a) => a,
            None => panic!("请传入对应的命令"),
        };
        let action = action.parse::<Action>();
        let action = match action {
            Ok(res) => res,
            _ => panic!("不支持的 action"),
        };
        if let Some(val) = args.get(2) {
            (action, Some(val.as_str()))
        } else {
            (action, None)
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::git_auto::cmd::Action;
    use std::env;

    #[test]
    fn test_action() {
        println!("{}", Action::NewFeature);
        println!("{:#?}", env::args());
    }
}
