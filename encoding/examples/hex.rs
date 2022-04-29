use data_encoding::{DecodeError, HEXUPPER};

fn main() -> Result<(), DecodeError> {
    // let original = b"The quick brown fox jumps over the lazy dog.";
    // let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\
    //     657220746865206C617A7920646F672E";

    // let encoded = HEXUPPER.encode(original);
    // assert_eq!(encoded, expected);

    // let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    // assert_eq!(&decoded[..], &original[..]);

    demo()?;

    Ok(())
}

// fn demo() {
fn demo() -> Result<(), DecodeError> {
    // let text = b"Article text";
    // let encoded = HEXUPPER.encode(text);
    let encoded = encode("❤Article text❤");
    println!("encoded: {}", encoded);

    // let tmp = &encoded.into_bytes()[..];
    // let decoded = HEXUPPER.decode(tmp)?;
    // let s = String::from_utf8_lossy(&decoded[..]);
    // let decoded = s.to_owned();

    let decoded = decode(encoded);

    println!("decoded: {}", decoded);
    Ok(())
}

pub fn encode(str: &str) -> String {
    HEXUPPER.encode(str.as_bytes())
}

pub fn decode(str: String) -> String {
    let tmp = &str.into_bytes()[..];
    let decoded = HEXUPPER.decode(tmp);
    if let Ok(t) = decoded {
        let s = String::from_utf8_lossy(&t[..]);
        s.to_string()
    } else {
        format!("Can not decode str: {}", String::from_utf8_lossy(tmp))
    }
}
