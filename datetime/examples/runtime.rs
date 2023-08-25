use std::time::Instant;

fn main() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("函数运行时间: {:?}", duration);
}

fn expensive_function() {
    println!("Hello datetime!");
}

