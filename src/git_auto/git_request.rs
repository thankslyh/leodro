use reqwest::header::HeaderMap;
use reqwest::{Client, Method, RequestBuilder, Url};
use serde::de::DeserializeOwned;

pub struct GitRequestClient<'a> {
    origin: &'a str,
    token: &'a str,
    pub client: Client,
}

impl<'a> GitRequestClient<'a> {
    pub fn new(origin: &'a str, token: &'a str) -> Self {
        GitRequestClient {
            origin,
            token,
            client: Client::new(),
        }
    }
    pub async fn request<T: DeserializeOwned>(
        &mut self,
        method: Method,
        path: &str,
        query: Option<&str>,
        params: Option<&str>,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let url = self.create_url(path, query, params);
        let mut head_map = HeaderMap::new();
        head_map.insert("PRIVATE-TOKEN", self.token.parse().unwrap());
        if method == Method::GET {
            let res = self.get(url.as_str()).headers(head_map).send().await?;
            Ok(res.json::<T>().await?)
        } else {
            let res = self.post(url.as_str()).headers(head_map).send().await?;
            Ok(res.json::<T>().await?)
        }
    }

    fn get(&mut self, url: &str) -> RequestBuilder {
        self.client.get(url)
    }
    fn post(&mut self, url: &str) -> RequestBuilder {
        self.client.post(url)
    }
    fn create_url(&mut self, path: &str, query: Option<&str>, params: Option<&str>) -> Url {
        let url = self.origin.parse::<Url>();
        let mut url = match url {
            Ok(url) => url,
            Err(e) => panic!("url 解析错误:{}", e),
        };
        url.set_path(path);
        if let Some(params) = params {
            url.set_path(params)
        }
        url.set_query(query);
        url
    }
}
