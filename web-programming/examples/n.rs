use std::ops::{Add, AddAssign};

fn main() {
    let n = 2.add(10);
    let n2 = 2.add_assign(100);

    println!("{:?}, {:?}", n, n2);
}