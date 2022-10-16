use csv::Error;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,

    // Option 类型要么值为 None, 要么值为 Some(value)
    // 如果 Some(value) 不能解析为一个正确的数字, 那么解析函数会抛出一个错误
    // 在这种情况下, 用 deserialize_with 指定一个替代处理函数来避免错误
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

use csv::ReaderBuilder;

fn main() -> Result<(), Error> {
    let data = "name\tplace\tid
        Mark\tMelbourne\t46
        Ashley\tZurich\t92
        Ashley\tZurich\t9a2";

    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(data.as_bytes());
    for object in reader.deserialize::<Record>().flatten() {
        println!(
            "id: {:?}, name: {:?}, place: {:?}",
            object.id, object.name, object.place
        );
    }

    Ok(())
}
