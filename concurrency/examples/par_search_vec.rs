// 并行处理数组元素

use rayon::prelude::*;

fn main() {
    let _ = data();
}

fn data() -> (Option<i32>, Option<i32>, Option<i32>) {
    let v = vec![6, 2, 1, 9, 3, 8, 11];

    // IntoParallelRefIterator
    // 引用并行迭代器
    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    (Some(*f1.unwrap()), Some(*f2.unwrap()), Some(*f3.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let (f1, f2, f3) = data();
        assert_eq!(f1, Some(9));
        assert_eq!(f2, Some(8));
        assert!(f3 > Some(8));
    }
}
