use chrono::{DateTime, Duration, Local};

fn day_earlier(datetime: DateTime<Local>) -> Option<DateTime<Local>> {
    datetime.checked_sub_signed(Duration::days(1))
}

fn main() {
    let now = Local::now();
    println!("现在: {}", now);

    let three_weeks = now
        .checked_add_signed(Duration::weeks(3))
        .and_then(|dur| dur.checked_add_signed(Duration::weeks(1)))
        .and_then(day_earlier);

    match three_weeks {
        Some(x) => println!("三周后:{}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("我们不能用计时器来告诉太阳系围绕银河系中心完成一个以上的完整轨道的时间。"),
    }
}
