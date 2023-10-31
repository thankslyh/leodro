use crate::git_auto::error::CustomError;
use regex::Regex;
use std::fs::File;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;
use std::{env, fs};

const CONFIG_ADDR: &'static str = ".git_auto/conf.toml";
const GIT_ADDR: &'static str = ".git/config";

#[derive(Debug, Default)]
pub struct Config {
    pub token: String,
    pub git_prefix: String,
    pub address: String,
    pub proj_name: String,
}
impl Config {
    pub fn new() -> Self {
        Config::default()
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
        self.address = path.to_str().unwrap().to_string();
        let mut file = File::open(&path).map_err(|_| CustomError::IoError)?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|_| CustomError::IoError)?;
        self.parse(&content)?;
        Ok(())
    }
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

    fn parse(&mut self, content: &str) -> Result<(), CustomError> {
        self.parse_git();
        let res = match content.parse::<toml::Value>() {
            Ok(toml::Value::Table(table)) => table,
            Ok(val) => {
                let err = format!("expected a table, found {}", val.type_str());
                println!("{}", err);
                panic!("{}", err);
            }
            Err(e) => {
                panic!(
                    "toml 解析错误，配置文件地址：{}下查看 toml 的内容格式是否正确\n错误信息：{}",
                    self.address, e
                )
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
            self.token = token;
            self.git_prefix = git_prefix;
            return Ok(());
        }
        Err(CustomError::ParseError("toml获取config为 None"))
    }

    pub fn parse_git(&mut self) {
        let dir = env::current_dir().unwrap();
        let dir = dir.join(GIT_ADDR);
        let mut f = match File::open(dir) {
            Ok(f) => f,
            Err(e) => panic!("从.git中解析项目错误，错误信息：{}", e),
        };
        let mut content = String::new();
        f.read_to_string(&mut content).unwrap();
        let re = Regex::new(r"/([^/]+)\.git").unwrap();
        let ca = match re.captures(content.as_str()) {
            Some(ca) => ca,
            None => panic!("从.git中解析项目错误"),
        };
        let repo_name = match ca.get(1) {
            Some(re) => re,
            None => panic!("从.git中解析项目错误"),
        };
        self.proj_name = repo_name.as_str().to_string();
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

#[cfg(test)]
mod tests {
    use crate::Config;
    use std::env;

    #[test]
    fn test_dir() {
        let mut conf = Config::new();
        conf.parse_git();
    }
}
