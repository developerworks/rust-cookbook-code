use error_chain::error_chain;
use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use url::{Position, Url};

// 错误链: 函数调用链中所有可能返回的错误集中处理
// 可能抛出错误的函数调用可以用 ? 后缀的形式传递错误,而不会导致编译器错误
error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        UrlParseError(url::ParseError);
        JoinError(tokio::task::JoinError);
    }
}

// 获取 Base 地址
async fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
    let base_tag_href = doc // 文档对象引用
        .find(Name("base")) // <base> 标签
        .filter_map(|n| n.attr("href")) // 节点对象
        .next();

    // Option.map_or_else(func_None, func_Some)
    // 对 Option 类型的值指定两个函数,
    // - 如果 Option 值为 None 运行第一个函数
    // - 如果 Option 值为 Some 运行第二个函数
    let base_url =
        base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
    Ok(base_url)
}

/// 检查连接的响应状态码
async fn check_link(url: &Url) -> Result<bool> {
    let res = reqwest::get(url.as_ref()).await?;
    Ok(res.status() != StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse("https://www.rust-lang.org/en-US/")?;
    let res = reqwest::get(url.as_ref()).await?.text().await?;
    // 文档对象
    let document = Document::from(res.as_str());
    let base_url = get_base_url(&url, &document).await?;

    println!("BASE URL: {}", base_url);

    // 创建一个带选项的解析器, 所有地址相对于 base 进行解析
    let base_parser = Url::options().base_url(Some(&base_url));

    // 收集页面的所有URL, 并存储到 HashSet 中
    let links: HashSet<Url> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href")) // a 的 href 值
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();

    // 并发线程池
    let mut tasks = vec![];

    // 创建线程
    for link in links {
        tasks.push(tokio::spawn(async move {
            if check_link(&link).await.unwrap() {
                println!("{} is OK", link);
            } else {
                println!("{} is Broken", link);
            }
        }));
    }

    // 等待所有线程完成
    for task in tasks {
        task.await?
    }

    Ok(())
}
