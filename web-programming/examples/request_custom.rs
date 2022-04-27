use reqwest::header::HeaderMap;
use serde_json::value::Value;
use std::collections::HashMap;

async fn get() -> Result<HashMap<String, String>, reqwest::Error> {
    let res = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    Ok(res)
}

async fn post() -> Result<HashMap<String, Value>, reqwest::Error> {
    // post 请求要创建client
    let client = reqwest::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // 组装要提交的数据
    let mut data = HashMap::new();
    data.insert("user", "tangjz");
    data.insert("password", "dev-tang.com");

    // 发起post请求并返回
    let res = client
        .post("https://httpbin.org/post")
        .headers(headers)
        .json(&data)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?;
    Ok(res)
}

#[tokio::main]
async fn main() {
    if let Ok(resp) = get().await {
        println!("{:#?}", resp);
    }

    if let Ok(res) = post().await {
        println!("{:#?}", res);
    }
}
