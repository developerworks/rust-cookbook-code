use std::{sync::Mutex, thread::Result};

use lazy_static::lazy_static;

// 全局可变状态
lazy_static! {
    #[derive(Debug)]
    pub static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT
        .lock()
        .map_err(|_| "Failed to acquire MutexGuard")
        .unwrap();

    db.push(fruit.to_string());
    Ok(())
}
