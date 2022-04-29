//! 如下实例使用 form_urlencoded::byte_serialize
//! 将字符串编码为 application/x-www-form-urlencoded 表单语法,
//! 随后使用 form_urlencoded::parse 对其进行解码。这两个函数都返回迭代器,
//! 然后这些迭代器聚集为 String

use url::form_urlencoded::{byte_serialize, parse};

fn main() {
    // let urlencoded: String = byte_serialize("What is this".as_bytes()).collect();
    let urlencoded = encode("What is your name?");
    println!("urlencoded: {}", urlencoded);

    // let decoded: String = parse(urlencoded.as_bytes())
    //     .map(|(key, value)| {
    //     [key, value].concat()
    // }).collect();

    let decoded = decode(urlencoded);

    println!("decoded: {}", decoded);
}

#[inline]
pub fn encode(str: &str) -> String {
    byte_serialize(str.as_bytes()).collect()
}

#[inline]
pub fn decode(str: String) -> String {
    parse(str.as_bytes())
        .map(|(key, value)| [key, value].concat())
        .collect()
}
