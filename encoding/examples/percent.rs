//! 使用 percent-encoding crate 中的 `utf8_percent_encode`
//! 函数对输入字符串进行百分比编码（URL 编码）。
//! 解码使用 `percent_decode` 函数。
use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, CONTROLS};
use std::str::Utf8Error;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn main() -> Result<(), Utf8Error> {
    let input = "confident, productive systems programming";
    println!(" origin: {}", input);

    // URL编码
    // let iter = utf8_percent_encode(input, FRAGMENT);
    // let encoded: String = iter.collect();
    let encoded = encode(input);
    assert_eq!(encoded, "confident,%20productive%20systems%20programming");
    println!("encoded: {}", encoded);

    // URL解码
    // let iter = percent_decode(encoded.as_bytes());
    // let decoded = iter.decode_utf8()?;
    let decoded = decode(encoded);
    assert_eq!(decoded, "confident, productive systems programming");
    println!("decoded: {}", decoded);

    Ok(())
}

pub fn encode(s: &str) -> String {
    utf8_percent_encode(s, FRAGMENT).collect()
}

pub fn decode(s: String) -> String {
    let decoded = percent_decode(s.as_bytes()).decode_utf8();

    if let Ok(s) = decoded {
        s.to_string()
    } else {
        String::from("Can not decode str: ") + &s
    }
}
