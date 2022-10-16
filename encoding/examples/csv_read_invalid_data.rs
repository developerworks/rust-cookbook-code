// CSV 文件通常包含无效数据。对于这些情形,
// csv crate 提供了一个自定义的反序列化程序 csv::invalid_option，它自动将无效数据转换为 None 值.
use csv::Error;
use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

fn main() -> Result<(), Error> {
    let data = "name,place,id
mark,sydney,46.5
ashley,zurich,92
akshat,delhi,37
alisha,colombo,xyz";

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
