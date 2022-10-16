use error_chain::error_chain;
use serde::Serialize;

use std::io;

error_chain! {
    foreign_links {
        CSVError(csv::Error);
        IOError(std::io::Error);
   }
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    #[serde(rename = "mail")]
    email: String
}

fn main() -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    // Header
    wtr.write_record(&["Id", " 名字", "邮件地址"])?;

    // Rows
    wtr.serialize((1, "王微微", "wangweiwei@163.com"))?;
    wtr.serialize((2, "沙漠屠夫", "shamotufu@qq.com"))?;
    wtr.serialize((3, "张小嘎", "zhangxiaoga@gmail.com"))?;

    // 被序列化的记录也可以是一个结构体
    wtr.serialize(User {
        id: 4,
        name: "王二小".to_string(),
        email: "wangerxiao@gmail.com".to_string()
    })?;

    // 还可以是一个 Vec
    let v = vec!["5".to_string(), "李冰冰".to_string(), "libingbing@qq.com".to_string()];
    wtr.serialize(v)?;

    wtr.flush()?;
    Ok(())
}
