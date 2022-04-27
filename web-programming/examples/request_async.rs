// use error_chain::error_chain;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let res = reqwest::get("http://httpbin.org/get").await?;
//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());

//     let body = res.text().await?;
//     println!("Body:\n{}", body);
//     Ok(())
// }

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder()
        .build()?;

    // Perform the actual execution of the network request
    let res = client
        .get("https://httpbin.org/ip")
        .send()
        .await?;

    // Parse the response body as Json in this case
    let ip = res
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:?}", ip);
    Ok(())
}
