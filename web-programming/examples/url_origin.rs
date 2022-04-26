use error_chain::error_chain;

use url::{Host, Origin, Url};

error_chain! {
    foreign_links {
        UrlParse(url::ParseError);
    }
}

fn main() -> Result<()> {
    let s = "ftp://rust-lang.org/examples";

    let url = Url::parse(s)?;

    // Build full URL with components
    let expected_scheme = "ftp".to_owned();
    let expected_host = Host::Domain("rust-lang.org".to_owned());
    let expected_port = 21;
    let expected = Origin::Tuple(expected_scheme, expected_host, expected_port);

    let origin = url.origin();
    // assert_eq!(origin, expected);
    // println!("The origin is as expected!");

    println!("     url: {:?}", s);
    println!("  origin: {:?}", origin);
    println!("expected: {:?}", expected);

    Ok(())
}
