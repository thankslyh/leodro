use leodro::Action;
use std::env;

#[tokio_macros::main]
async fn main() -> leodro::service::ServiceResult {
    println!("{:?}", env::current_dir());
    let env = env::args().collect::<Vec<String>>();
    let cmd = leodro::cmd::Cmd::new(&env);
    let mut conf = leodro::Config::new();
    conf.read_config().expect("TODO: panic message");
    let mut git_req = leodro::GitRequestClient::new(&conf.git_prefix, &conf.token);
    match cmd.action {
        Action::NewFeature => {
            leodro::service::new_feature(&mut git_req, conf.proj_name.as_str(), cmd.val.unwrap())
                .await?;
        }
        Action::Issues => {
            leodro::service::issues(&mut git_req, conf.proj_name.as_str(), cmd.val).await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use reqwest::Url;
    use spinners::Spinner;
    use spinners::Spinners::SoccerHeader;
    use std::env;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_url() {
        println!("env:{:#?}", dirs::home_dir());
        let url = "https://git.shuiditech.com".parse::<Url>();
        if let Ok(url) = url {
            println!("{}", url)
        }
    }

    #[test]
    fn test_spinner() {
        let mut sp = Spinner::new(SoccerHeader, "".to_string());
        sleep(Duration::from_secs(5));
        sp.stop();
    }
}
