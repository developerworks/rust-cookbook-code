use std::collections::HashMap;

use reqwest::Result;
// use serde::Deserialize;
// use web_programming::{Dependency, Meta, Version};
use serde::{Deserialize, Serialize};

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

struct ReverseDependencies {
    crate_id: String,
    dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
    client: reqwest::blocking::Client,
    page: u64,
    per_page: u64,
    total: u64,
    versions: <Vec<Version> as IntoIterator>::IntoIter,
}

impl ReverseDependencies {
    fn of(crate_id: &str) -> Result<Self> {
        Ok(ReverseDependencies {
            crate_id: crate_id.to_owned(),
            dependencies: vec![].into_iter(),
            client: reqwest::blocking::Client::new(),
            page: 0,
            per_page: 1,
            total: 0,
            versions: vec![].into_iter(),
        })
    }

    #[allow(unused)]
    fn try_next(&mut self) -> Result<Option<Dependency>> {
        if let Some(dep) = self.dependencies.next() {
            return Ok(Some(dep));
        }

        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        self.page += 1;
        let url = format!(
            "https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
            self.crate_id, self.page, self.per_page
        );

        // Localhost is OK!
        // let url = "http://localhost:8888/webapi_pagination.json".to_owned(); // python3 -m http.server 8888

        // or Online version!
        let url = "https://raw.githubusercontent.com/developerworks/rust-cookbook-code/main/web-programming/examples/webapi_pagination.json".to_owned();

        println!("url: {}", url);

        let response = self.client.get(&url).send()?.json::<ApiResponse>()?;

        // Why can not access crates.io behand Cloudflare, cause by: Decode error
        // Reference to: examples/request_json_test.rs
        println!("response: {:?}", response);

        self.dependencies = response.dependencies.into_iter();
        self.total = response.meta.total;
        self.versions = response.versions.into_iter();
        Ok(self.dependencies.next())

        // Ok(None)
    }
}

impl Iterator for ReverseDependencies {
    type Item = Result<Dependency>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(dep)) => Some(Ok(dep)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

fn main() -> Result<()> {
    for dep in ReverseDependencies::of("serde")? {
        println!("reverse dependency: {}", dep?.crate_id);
    }
    Ok(())
}
