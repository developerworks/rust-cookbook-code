use csv::Error;

fn main() -> Result<(), Error> {

    // 逗号分割值
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    // 构造 Reader 对象
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    
    // 迭代
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0], &record[1], &record[2], &record[3]
        );
    }

    Ok(())
}
