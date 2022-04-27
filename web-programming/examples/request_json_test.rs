use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub dependencies: Vec<Dependency>,
    pub meta: Meta,
    pub versions: Vec<Version>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub crate_id: String,
    pub default_features: bool,
    pub downloads: u64,
    pub features: Vec<String>,
    pub id: u64,
    pub kind: String,
    pub optional: bool,
    pub req: String,
    pub target: Option<serde_json::Value>,
    pub version_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub total: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub audit_actions: Vec<AuditAction>,
    #[serde(rename = "crate")]
    pub version_crate: String,
    pub crate_size: u64,
    pub created_at: String,
    pub dl_path: String,
    pub downloads: u64,
    pub features: HashMap<String, Vec<String>>,
    pub id: u64,
    pub license: String,
    pub links: Links,
    pub num: String,
    pub published_by: PublishedBy,
    pub readme_path: String,
    pub updated_at: String,
    pub yanked: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditAction {
    pub action: String,
    pub time: String,
    pub user: PublishedBy,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PublishedBy {
    pub avatar: String,
    pub id: u64,
    pub login: String,
    pub name: String,
    pub url: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub authors: String,
    pub dependencies: String,
    pub version_downloads: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder().build()?;

    // Perform the actual execution of the network request
    let res = client
        // .get("http://localhost:8888/webapi_pagination.json") // python3 -m http.server 8888
        .get("https://raw.githubusercontent.com/developerworks/rust-cookbook-code/main/web-programming/examples/webapi_pagination.json")
        .send()
        .await?;

    println!("{:?}", res);

    // Parse the response body as Json in this case
    let res = res.json::<ApiResponse>().await?;

    println!("{:?}", res);
    // println!("origin: {}", meta.total);
    Ok(())
}
