use ring::{hmac, rand};
use ring::error::Unspecified;
use ring::rand::SecureRandom;

///////////////////////////////////
// 使用 HMAC 摘要对消息进行签名和验证
///////////////////////////////////
fn main() -> Result<(), Unspecified> {
    // 秘钥值
    let mut key_value = [0u8; 48];
    // 随机天聪
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;

    // 创建秘钥结构
    // pub struct Key {
    //     inner: digest::BlockContext,
    //     outer: digest::BlockContext,
    // }
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    // =======
    // 待签名消息
    // =======
    let message = "Legitimate and important message.";
    // =======
    // 签名
    // =======
    let signature = hmac::sign(&key, message.as_bytes());
    // =======
    // 验证
    // =======
    hmac::verify(&key, message.as_bytes(), signature.as_ref())

    // Ok(())
}
