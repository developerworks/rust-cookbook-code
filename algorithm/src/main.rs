use rand::{self, Rng};
// 均匀分布
use rand::distributions::{Distribution, Standard, Uniform};

mod rand_password;
mod sortby;

#[derive(Debug)]
struct Point {
    _x: u8,
    _y: u8,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        // 自动推导?
        let (rand_x, rand_y): (u8, u8) = rng.gen();
        Point {
            _x: rand_x,
            _y: rand_y,
        }
    }
}

fn main() {
    let mut random = rand::thread_rng();

    let n: u8 = random.gen();
    let n2: u16 = random.gen();

    // 随机数
    println!("随机数 u8: {}", n);
    println!("随机数 u16:{}", n2);
    println!("随机数 i32: {}", random.gen::<u32>());
    println!("随机数 u32: {}", random.gen::<u32>());
    println!("随机数 f64: {}", random.gen::<f64>());

    /*
    自定义范围
    */

    println!("随机数范围 integer: {}", random.gen_range(0i32..10i32));
    println!("随机数范围 float: {}", random.gen_range(0.0f64..10.0f64));

    // 均匀分布: 丢色子, 直到丢到 6 点, 退出循环
    let die: Uniform<i32> = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut random);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }

    // 自定义类型随机
    let pt: Point = random.gen::<Point>();
    println!("随机三元组: {:?}", random.gen::<(i32, bool, f64)>());
    println!("随机点: {:?}", pt);

    // 元组最多12元素
    let multi_rand = random.gen::<(u8, u8, u8)>();
    println!("一次生成多个随机数: {:?}", multi_rand);

    let pass: String = rand_password::generate(32);
    println!("随机密码: {}", pass);

    let pass: String = rand_password::generate_from_chars(32);
    println!("随机密码: {}", pass);

    let v: &mut Vec<i32> = &mut vec![1, 3, 4, 5, 9, 0, 2];
    // v.sort();
    sortby::sort(v);
    println!("Vec sorted: {:?}", v);

    // let v_float = vec![1.2, 3.1, 0.7, 1.0];
    // let sorted_float = orderby::sort_by(v_float);
    // println!("Vec sorted float: {:?}", sorted_float);

    sortby::sort_struct();
}
