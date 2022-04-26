use chrono::{DateTime, FixedOffset, Utc, Local};

fn main() {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east(8*3600);
    let rio_timezone = FixedOffset::west(2*3600);
    println!("Local time now is {}", local_time);
    println!("Utc time is {}", utc_time);
    println!("Hongkong time is {}", utc_time.with_timezone(&china_timezone));
    println!("Rio time is {}", utc_time.with_timezone(&rio_timezone));
}