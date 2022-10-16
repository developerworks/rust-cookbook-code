use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("{}", request_url);
    // reqwest::get 属于一次性调用, 每次调用内部都会创建 Client,
    // 因此在需要重复使用 Client 的情况下, 需要手工构造, 例如:
    // let client = Client::builder().build(); 等效于 ClientBuilder::new()
    // let client = Client::new()
    let response = reqwest::get(&request_url).await?;
    println!("{:?}", response);

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
