use std::borrow::Borrow;

use url::{ParseError, Position, Url};

fn main() -> Result<(), ParseError> {
    let url = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";
    let parsed = Url::parse(url)?;

    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("  orgin url: {}", url);
    println!("cleaned url: {}", cleaned);

    // Slice the url by Position index
    // The position index is compute by slicing.rs module from url crate
    // You can access url components with position index
    // For example:

    // schema
    let schema: &str = &parsed[..Position::AfterScheme];
    println!("schema: {}", schema);
    // host
    let host: &str = &parsed[..Position::AfterHost];
    println!("  host: {}", host);
    // or Just domain
    // This cut by start position and end position of host
    let domain: &str = &parsed[Position::BeforeHost..Position::AfterHost];
    println!("domain: {}", domain);

    // path
    let path: &str = &parsed[Position::BeforePath..Position::AfterPath];
    println!("path: {}", path);

    // Query string
    let qs: &str = &parsed[Position::BeforeQuery..Position::AfterQuery];
    println!("query string: {}", qs);

    // I get a point, use Url to parse mysql database connection string to connection params
    let params = Url::parse("mysql://root:root@localhost/database?characterEncoding=utf-8")?;
    println!("  schema: {},\nusername: {},\npassword: {},\n    host: {},\ndatabase: {}\n", 
        params.scheme(), 
        params.username(), 
        params.password().unwrap(), 
        params.host().unwrap(), 
        params.path().get(1..).unwrap()
    );
    for p in params.query_pairs() {
        println!("propertiy pair: {:?}, key = {}, value = {}", p.borrow(), p.borrow().0, p.borrow().1);
    }
    


    Ok(())
}
