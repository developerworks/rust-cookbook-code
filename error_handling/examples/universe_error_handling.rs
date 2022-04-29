use error_chain::error_chain;
use reqwest::blocking::{self, Response};
use std::{io::Read, str::FromStr};
use url::Url;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        ParseIntError(std::num::ParseIntError);
        UrlParse(url::ParseError);
    }
    errors { RandomResponseError(t: String) }
}

fn parse_response(mut response: Response) -> Result<u32> {
    // String buffer
    let mut body = String::new();

    // Read response text to string buffer
    response.read_to_string(&mut body)?;

    // Removes the last character
    body.pop();

    // Parse number
    body.parse::<u32>()
        .chain_err(|| ErrorKind::RandomResponseError(body))
}

// #[rustfmt::skip]
fn run() -> Result<()> {
    // 静态
    #[allow(unused)]
    let url = "https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain"
        .to_string();
    // 动态构造, 适合于需要动态创建的 URL
    let dynamic_base = Url::parse("https://www.random.org")?;
    let mut dynamic_url = dynamic_base.join("integers")?;
    dynamic_url
        .query_pairs_mut()
        .append_pair("num", "1")
        .append_pair("min", "0")
        .append_pair("max", "100")
        .append_pair("col", "1")
        .append_pair("base", "10")
        .append_pair("format", "plain");

    let dynamic_s = dynamic_url.as_str();
    println!("dynamic construct url: {}", dynamic_s);
    let response = blocking::get(dynamic_s)?;
    let random_value: u32 = parse_response(response)?;
    println!("a random number between 0 and 10: {}", random_value);
    Ok(())
}

#[rustfmt::skip]
fn main() -> Result<()> {
    // 统一错误处理    
    if let Err(error) = run() {        
        match *error.kind() {            
            ErrorKind::Io(_)                    => println!("Standard IO error: {:?}", error),
            ErrorKind::Reqwest(_)               => println!("Reqwest error: {:?}", error),
            ErrorKind::ParseIntError(_)         => println!("Standard parse int error: {:?}", error),
            ErrorKind::RandomResponseError(_)   => println!("User defined error: {:?}", error),
            _                                   => println!("Other error: {:?}", error),
        }
    }
    Ok(())
}

#[allow(unused)]
fn demo() -> Result<()> {
    // Rust 2021 的新写法
    for i in [1, 2, 3, 4, 5, 6, 7, 8, 9] {
        println!("array element: {}", i);
    }


    // 通过 from_str() 构造 URL
    // from_str() 内部使用 Url::parse()
    let mut url = Url::from_str("https://localhost/index")?;
    let u = url
        .query_pairs_mut()
        .append_pair("name", "value")
        .finish()
        .as_str();

    println!("url: {}", u);

    Ok(())
}
