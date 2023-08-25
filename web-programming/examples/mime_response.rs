use std::str::FromStr;

// https://rustwiki.org/zh-CN/rust-cookbook/web/mime.html#%E8%A7%A3%E6%9E%90-http-%E5%93%8D%E5%BA%94%E7%9A%84-mime-%E7%B1%BB%E5%9E%8B
use error_chain::error_chain;
use mime::Mime;
use reqwest::header::CONTENT_TYPE;

error_chain! {
   foreign_links {
       Reqwest(reqwest::Error);
       Header(reqwest::header::ToStrError);
       Mime(mime::FromStrError);
   }
}

#[tokio::main]
async fn main() -> Result<()> {
    let response = reqwest::get("https://www.rust-lang.org/logos/rust-logo-32x32.png").await?;
    let headers = response.headers();

    match headers.get(CONTENT_TYPE) {
        None => {
            println!("The response does not contain a Content-Type header.");
        }
        Some(content_type) => {
            let content_type = Mime::from_str(content_type.to_str()?)?;
            let media_type = match (content_type.type_(), content_type.subtype()) {
                (mime::TEXT, mime::HTML) => "a HTML document",
                (mime::TEXT, _) => "a text document",
                (mime::IMAGE, mime::PNG) => "a PNG image",
                (mime::IMAGE, _) => "an image",
                _ => "neither text nor image",
            };

            println!("The reponse contains {}.", media_type);
        }
    };

    Ok(())
}
