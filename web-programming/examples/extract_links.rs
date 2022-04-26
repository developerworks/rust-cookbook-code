use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

#[tokio::main]
// #[tokio::main(flavor = "current_thread")]
async fn run() -> Result<()> {
    // HTML字符串
    let res = reqwest::get("https://www.rust-lang.org/en-US/")
        .await?
        .text()
        .await?;

    // DOM
    Document::from(res.as_str())
        .find(Name("a"))
        // .filter_map(|n| n.attr("href"))
        .filter_map(|n| {
            // 只包括 http 开头的
            if let Some(href) = n.attr("href") {
                if href.len() >= 4 && &href[0..4] == "http" {
                    Some(href)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .for_each(|x| println!("{}", x));

    Ok(())
}

fn main() -> Result<()> {
    run()
}
