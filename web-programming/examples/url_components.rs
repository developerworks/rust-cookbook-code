
use url::{Url, Host, ParseError};

fn main() -> Result<(), ParseError> {
    let s = "ftp://rust-lang.org/examples";

    let url = Url::parse(s)?;

    assert_eq!(url.scheme(), "ftp");
    assert_eq!(url.host(), Some(Host::Domain("rust-lang.org")));
    assert_eq!(url.port_or_known_default(), Some(21));
    println!("The origin is as expected!");

    println!("schema: {}", url.scheme());
    println!("  host: {}", url.host().unwrap());
    println!("  port: {}", url.port_or_known_default().unwrap());

    Ok(())
}
