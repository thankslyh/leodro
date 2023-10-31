use std::env;
use thankslyh_toml::Action;

#[tokio_macros::main]
async fn main() -> thankslyh_toml::service::ServiceResult {
    let env = env::args().collect::<Vec<String>>();
    let cmd = thankslyh_toml::cmd::Cmd::new(&env);
    let mut conf = thankslyh_toml::Config::new();
    conf.read_config().expect("TODO: panic message");
    let mut git_req = thankslyh_toml::GitRequestClient::new(&conf.git_prefix, &conf.token);
    if cmd.action == Action::NewFeature {
        let res =
            thankslyh_toml::service::new_feature(&mut git_req, "thankslyh_test", cmd.val.unwrap())
                .await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use reqwest::Url;
    use std::env;

    #[test]
    fn test_url() {
        println!("env:{:#?}", dirs::home_dir());
        let url = "https://git.shuiditech.com".parse::<Url>();
        if let Ok(url) = url {
            println!("{}", url)
        }
    }
}
