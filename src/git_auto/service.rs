use crate::GitRequestClient;
use reqwest::Method;
use serde::Deserialize;
use spinners::Spinner;
use spinners::Spinners::SoccerHeader;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct Project {
    pub id: u32,
    pub name: String,
    pub web_url: String,
}

#[derive(Debug, Deserialize)]
struct Issue {
    pub iid: u32,
    pub title: String,
    pub description: Option<String>,
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
    let mut sp = Spinner::new(SoccerHeader, format!("项目名 {} 创建 branch 中", proj_name));
    let r = c
        .request::<Vec<Project>>(
            Method::GET,
            "/api/v4/projects",
            Some(&format!("search={}", proj_name)),
            None,
        )
        .await?;
    let first = r.iter().find(|p| p.name.eq(proj_name)).unwrap();
    let tmp_title = "title=".to_owned() + title;
    let query = Some(tmp_title.as_str());
    let i = c
        .request::<Issue>(
            Method::POST,
            &format!("api/v4/projects/{}/issues", first.id),
            query,
            None,
        )
        .await?;
    let tmp_branch = format!("issue#{}", i.iid);
    let branch = format!("ref=master&branch={}", tmp_branch);
    let branch = Some(branch.as_str());
    c.request::<Branch>(
        Method::POST,
        &format!("/api/v4/projects/{}/repository/branches", first.id),
        branch,
        None,
    )
    .await?;
    Command::new("git")
        .arg("fetch")
        .output()
        .expect("命令执行错误，请手动拉取分支");
    Command::new("git")
        .args(["checkout", &tmp_branch])
        .output()
        .expect("分支切换错误，请手动切换");
    sp.stop_with_newline();
    println!("分支已创建成功，分支名{}", tmp_branch);
    Ok(())
}

pub async fn issues<'a>(c: &'a mut GitRequestClient<'a>, proj_name: &str) -> ServiceResult {
    let mut sp = Spinner::new(SoccerHeader, format!("项目名 {} 查找 issues 中", proj_name));
    let r = c
        .request::<Vec<Project>>(
            Method::GET,
            "/api/v4/projects",
            Some(&format!("search={}", proj_name)),
            None,
        )
        .await?;
    let first = r.iter().find(|p| p.name.eq(proj_name)).unwrap();
    let list = c
        .request::<Vec<Issue>>(
            Method::GET,
            &format!("/api/v4/projects/{}/issues", first.id),
            None,
            None,
        )
        .await?;
    sp.stop_with_newline();
    println!("issue_id    title                          description\r\n");
    list.iter().for_each(|v| {
        println!(
            "{}    {}                                    {}\r",
            format!("issue#{}", v.iid),
            v.title,
            v.description.clone().unwrap_or_else(|| String::new())
        )
    });
    Ok(())
}
