use rand::{Rng, thread_rng};
use rand::distributions::{Alphanumeric, Distribution};

#[inline]
pub fn generate(len: usize) -> String {
    let rng = thread_rng();
    // let v: Vec<f32> = Standard.sample_iter(rng).take(16).collect();
    let v: Vec<u8> = Alphanumeric.sample_iter(rng).take(len).collect();
    let s = String::from_utf8(v);
    if let Ok(s) = s {
        s
    } else {
        "".to_owned()
    }
}

#[inline]
#[rustfmt::skip]
pub fn generate_from_chars(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~";
    let mut rng = thread_rng();
    (0..len).map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        char::from(unsafe {
            // 这里边界是已知的, 并且在 CHARSET 范围内, 安全!
            *CHARSET.get_unchecked(idx)
        })
    }).collect()
}
