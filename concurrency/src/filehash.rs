use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::Path;

use ring::digest::{Context, Digest, SHA256};

pub fn _compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
    let mut buf_reader = BufReader::new(File::open(&filepath)?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok((context.finish(), filepath))
}

pub fn _is_iso(path: &std::path::Path) -> bool {
    path.ends_with(".iso")
}
