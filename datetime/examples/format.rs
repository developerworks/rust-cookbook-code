use chrono::Local;

const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const DATE_FORMAT: &str = "%Y-%m-%d";
const TIME_FORMAT: &str = "%H:%M:%S";

fn main() {
    let now = Local::now();
    println!("格式 1: {}", now.format(DATE_TIME_FORMAT));
    println!("日期: {}", now.format(DATE_FORMAT));
    println!("时间: {}", now.format(TIME_FORMAT));
}
