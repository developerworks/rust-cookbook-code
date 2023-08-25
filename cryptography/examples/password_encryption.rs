use std::num::NonZeroU32;

use data_encoding::HEXUPPER;
use ring::{digest, pbkdf2, rand};
use ring::error::Unspecified;
use ring::rand::SecureRandom;

fn main() -> Result<(), Unspecified> {
    // 64位凭证输出长度
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();

    // 一个安全的其随机值直接来自于操作系统的随机数生成器
    let rng = rand::SystemRandom::new();

    // =======
    // 盐
    // =======
    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    // =======
    // 正确和错误密码
    // =======
    let password = "Guess Me If You Can!";
    let wrong_password = "Definitely not the correct password";

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

    // =======
    // 加密
    // =======
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("       Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    // =======
    // 验证成功
    // =======
    let should_succeed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &pbkdf2_hash,
    );

    // =======
    // 验证失败
    // =======
    let should_fail = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );

    assert!(should_succeed.is_ok());
    assert!(should_fail.is_err());

    Ok(())
}
