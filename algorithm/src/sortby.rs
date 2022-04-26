pub fn sort(v: &mut Vec<i32>) -> &Vec<i32> {
    // let mut v2 = v.clone();
    v.sort_unstable();
    v
}

// pub fn sort_by(&mut v: Vec<f32>) -> Vec<f32>{
//     v.sort_by(|a, b| a.partial_cmp(b).unwrap())
// }

pub fn sort_struct() {
    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        pub fn new(name: String, age: u32) -> Self {
            Person { name, age }
        }
    }

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // 自然顺序,排序 people (名字 和 年龄)
    people.sort();
    println!("自然顺序,排序 people (名字 和 年龄) {:?}", people);

    people.sort_by(|a, b| b.age.cmp(&a.age));
    println!("用年龄排序 {:?}", people);
}
