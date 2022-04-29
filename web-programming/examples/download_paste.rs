use error_chain::error_chain;
use std::fs::File;
use std::io::Read;

error_chain! {
    foreign_links {
        HttpRequest(reqwest::Error);
        IoError(::std::io::Error);
    }
}

fn main() -> Result<()> {
    // hello_paste();
    hello_paste_rust()
}

#[tokio::main]
#[allow(unused)]
async fn hello_paste() -> Result<()> {
    let paste_api = "https://paste.rs";
    let mut file = File::open("message")?;
    let mut buffer = String::new();
    // 把文件内容读取读取到 constants 变量

    let filesize = file.read_to_string(&mut buffer);

    // file.read_to_string(&mut constants);

    let client = reqwest::Client::new();
    let res = client.post(paste_api).body(buffer).send().await?;
    let response_text = res.text().await?;
    println!("代码块地址: {:?}, 文件大小 {:?}", response_text, filesize);

    Ok(())
}

#[tokio::main]
async fn hello_paste_rust() -> Result<()> {
    let url = "https://paste.rs/web";
    let mut file = File::open("examples/download_paste.rs")?;
    let mut buffer = String::new();
    let filesize = file.read_to_string(&mut buffer);

    // 创建请求对象
    let client = reqwest::Client::new();
    // client.b
    let res = client.post(url).body(buffer).send().await?;
    let response_text = res.text().await?;
    println!("代码块地址: {:?}, 文件大小 {:?}", response_text, filesize);
    Ok(())
}
