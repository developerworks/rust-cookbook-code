use std::{thread, time};

use crossbeam::channel::unbounded;

pub fn data() {
    let (sender, receiver) = unbounded::<i32>();
    let n_msgs = 10;
    crossbeam::scope(|scope| {
        // 子线程 None/ThreadId(1024)
        scope.spawn(|_| {
            for i in 1..=n_msgs {
                sender.send(i).unwrap();
                println!(
                    "{} sent by thread {:?}/{:?}",
                    i,
                    thread::current().name(),
                    thread::current().id()
                );
                thread::sleep(time::Duration::from_millis(100));
            }
        });
        // 主线程 Some("main")/ThreadId(1)
        for _ in 1..=n_msgs {
            let msg = receiver.recv().unwrap();
            println!(
                "thread name/id: {:?}/{:?}, received: {}",
                thread::current().name(),
                thread::current().id(),
                msg
            );
        }
    })
        .unwrap();
}
