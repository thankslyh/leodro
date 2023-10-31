use crate::git_auto::error::CustomError;
use std::fmt::{format, Pointer};
use std::fs;
use std::fs::File;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;

const CONFIG_ADDR: &'static str = ".git_auto/conf.toml";

#[derive(Debug)]
pub struct Config {
    pub token: String,
    pub git_prefix: String,
}
impl Config {
    pub fn new() -> Self {
        Config {
            token: "".to_string(),
            git_prefix: "".to_string(),
        }
    }
    pub fn read_config(&mut self) -> Result<(), CustomError> {
        let path = Config::get_conf_addr().map_err(|_| CustomError::PathNotFount)?;

        let path = match Config::check_url(&path) {
            Ok(_) => path,
            Err(CustomError::PathNotFount) => {
                println!(
                    "配置文件没有找到，请在改路径下{:?}创建配置或根据提示生成对应的配置文件",
                    path
                );
                match Config::gen_toml(&path) {
                    Ok(_) => path,
                    Err(_) => panic!("生成conf.toml错误"),
                }
            }
            _ => {
                panic!("")
            }
        };
        let mut file = File::open(&path).map_err(|_| CustomError::IoError)?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|_| CustomError::IoError)?;
        let res = Config::parse(&content)?;
        self.token = res.token;
        self.git_prefix = res.git_prefix;
        Ok(())
    }
    // fn create_file(path: &PathBuf) -> Result<(), CustomError> {
    //     let file = File::create(path);
    // }
    fn check_url(address: &PathBuf) -> Result<(), CustomError> {
        let meta = fs::metadata(address);
        if meta.is_ok() {
            return Ok(());
        }
        Err(CustomError::PathNotFount)
    }
    fn get_conf_addr() -> Result<PathBuf, CustomError> {
        let env = match dirs::home_dir() {
            Some(e) => e,
            None => return Err(CustomError::PathNotFount),
        };
        let address = env.join(CONFIG_ADDR);
        Ok(address)
    }

    fn parse(content: &str) -> Result<Config, CustomError> {
        let res = match content.parse::<toml::Value>() {
            Ok(toml::Value::Table(table)) => table,
            Ok(val) => {
                let err = format!("expected a table, found {}", val.type_str());
                println!("{}", err);
                panic!("{}", err);
            }
            Err(e) => {
                panic!("{}", e)
            }
        };
        if let Some(config) = res.get("config").cloned() {
            let token = config.get("token").unwrap().as_str().unwrap().to_string();
            let git_prefix = config
                .get("git_prefix")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            return Ok(Config { token, git_prefix });
        }
        Err(CustomError::IoError)
    }

    fn gen_toml(path: &PathBuf) -> Result<(), CustomError> {
        println!("{:?}", path.parent());
        let parent = match path.parent() {
            Some(p) => p,
            None => panic!("parent不存在{:?}", path),
        };
        fs::create_dir_all(parent).map_err(|_| CustomError::PathNotFount)?;
        println!("正在创建文件...{:?}", path);
        let mut f = match File::create(path) {
            Ok(f) => f,
            Err(e) => panic!("报错：{}", e),
        };
        let mut token = String::new();
        let mut prefix = String::new();
        loop {
            println!("请输入git accessToken：");
            stdin().read_line(&mut token).expect("读取 token错误");
            println!("请输入git lib api前缀,一般是：{}", "https://git.xxx.com");
            stdin().read_line(&mut prefix).expect("读取前缀错误");
            if !token.is_empty() && !prefix.is_empty() {
                break;
            }
        }
        // println!("token={} prefix={}", token.trim(), prefix.trim());
        let content = format!(
            "[config]\r\ntoken = \"{}\"\r\ngit_prefix = \"{}\"",
            token.trim(),
            prefix.trim()
        );
        f.write(&content.as_bytes())
            .map_err(|_| CustomError::PathNotFount)?;
        Ok(())
    }
}
