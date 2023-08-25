// #![feature(mutex_unlock)]

use std::{io::Error, sync::mpsc::channel};

use log::info;
use threadpool::ThreadPool;
use walkdir::WalkDir;

mod filehash;
mod find_max;
mod global_mutable_state;
mod pass_data;
mod pipe;

// use log4rs;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("Hello, world!");
    let arr = &mut [0; 1000];
    for (elem, val) in arr.iter_mut().zip(1..=1000) {
        *elem = val;
    }
    let max = find_max::find_max(arr).unwrap();
    dbg!("max number: {}", max);

    pipe::message();
    pass_data::data();

    global_mutable_state::insert("apple").unwrap();
    global_mutable_state::insert("orange").unwrap();
    global_mutable_state::insert("peach").unwrap();
    // {
    //     let db = global_mutable_state::FRUIT
    //         .lock()
    //         .map_err(|_| "Failed to acquire MutexGuard")
    //         .unwrap();

    //     db.iter()
    //         .enumerate()
    //         .for_each(|(i, item)| println!("{}: {}", i, item));
    // }
    global_mutable_state::insert("grape").unwrap();
    {
        let db = global_mutable_state::FRUIT
            .lock()
            .map_err(|_| "Failed to qcquire MutexGuard")
            .unwrap();
        db.iter()
            .enumerate()
            .for_each(|(i, item)| println!("{}: {}", i, item));
    }

    // Ok(())
}

fn _filehash() -> Result<(), Error> {
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();

    for entry in WalkDir::new("/home/user/Downloads")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() && filehash::_is_iso(e.path()))
    {
        let path = entry.path().to_owned();
        let tx = tx.clone();
        pool.execute(move || {
            let digest = filehash::_compute_digest(path);
            tx.send(digest).expect("Could not send data!");
        });
    }

    drop(tx);
    for t in rx.iter() {
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
    }
    Ok(())
}
