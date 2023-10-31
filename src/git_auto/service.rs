use crate::GitRequestClient;
use reqwest::Method;
use serde::Deserialize;
use std::future::Future;

#[derive(Debug, Deserialize)]
struct Project {
    pub id: u32,
}

#[derive(Debug, Deserialize)]
struct Issue {
    pub iid: u32,
}

#[derive(Debug, Deserialize)]
struct Branch {
    name: String,
}

pub type ServiceResult = Result<(), Box<dyn std::error::Error>>;
pub async fn new_feature<'a>(
    c: &'a mut GitRequestClient<'a>,
    proj_name: &str,
    title: &str,
) -> ServiceResult {
    let r = c
        .request::<Vec<Project>>(
            Method::GET,
            "/api/v4/projects",
            Some(&format!("search={}", proj_name)),
            None,
        )
        .await?;
    let first = r.get(0).unwrap();
    let tmp_title = "title=".to_owned() + title;
    let query = Some(tmp_title.as_str());
    println!("开始创建 issue");
    let i = c
        .request::<Issue>(
            Method::POST,
            &format!("api/v4/projects/{}/issues", first.id),
            query,
            None,
        )
        .await?;
    let branch = format!("ref=master&branch={}", format!("issue#{}", i.iid));
    let branch = Some(branch.as_str());
    println!("开始创建 分支");
    c.request::<Branch>(
        Method::POST,
        &format!("/api/v4/projects/{}/repository/branches", first.id),
        branch,
        None,
    )
    .await?;
    println!("分支已创建成功，分支名{}", branch.unwrap());
    Ok(())
}
