// 通过文件后缀名判断 Mime 类型
use mime::Mime;
use tabular::{Row, Table};

fn find_mimetype(filename: &str) -> Mime {
    let parts: Vec<&str> = filename.split('.').collect();

    match parts.last() {
        Some(v) => match *v {
            "png" => mime::IMAGE_PNG,
            "jpg" => mime::IMAGE_JPEG,
            "json" => mime::APPLICATION_JSON,
            &_ => mime::TEXT_PLAIN,
        },
        None => mime::TEXT_PLAIN,
    }
}

fn main() {
    // {:<} left-align
    // {:^} center-align
    // {:>} right-align
    let mut table = Table::new("{:<} {:<}");
    table.add_row(Row::new().with_cell("FILE").with_cell("MIME"));

    let filenames = vec!["foobar.jpg", "foo.bar", "foobar.png"];
    for file in filenames {
        let mime = find_mimetype(file);
        table.add_row(Row::new().with_cell(file).with_cell(mime));

        // println!("MIME for {}: {}", file, mime);
    }
    println!("{}", table);
}
