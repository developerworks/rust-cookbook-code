use std::fs::File;
use std::io::{BufReader, Read, Result, Write};

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};

//
fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok(context.finish())
}

/////////////////////////////////////
/// 计算文件的 SHA-256 摘要
///////////////////////////////////// 
fn main() -> Result<()> {
    let path = "timg.txt";

    let mut output = File::create(path)?;
    write!(output, "We will generate a digest of this text")?;

    let file = File::open(path)?;

    // 创建一个默认缓冲区, 大小为 8KB, 这里是文件缓冲区,
    // BufReader 所包裹的对象必须实现 Read trait
    let reader = BufReader::new(file);
    let digest = sha256_digest(reader)?;

    println!(
        "   File: {}\nSHA-256: {}",
        path,
        HEXUPPER.encode(digest.as_ref())
    );

    Ok(())
}
