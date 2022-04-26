use crossbeam::channel::bounded;
use std::thread;
use std::time::Duration;

// use rand::Rng;
// use std::cmp;
// use std::fs::File;
// use std::io::{BufWriter, Write};
// https://users.rust-lang.org/t/idiomatic-way-to-generate-a-file-of-a-given-size/30407/2

// use std::io::ErrorKind;

pub fn message() {
    let (snd1, rcv1) = bounded::<i32>(1);
    let (snd2, rcv2) = bounded::<i32>(1);
    let n_msgs = 10;
    let n_workers = 2;

    // TOTO::
    // 1. 主线程获取一个文件的大小
    // 2. 依据文件大小设为 T 等分 N-1 个大小为 S 的块, 第 N 块大小为 T-(N-1)*S
    // 3. 构造 N 个请求, 使用 HTTP Range 分块下载文件
    // 4. 文件存储方法
    //      4.1 有多少个块, 创建多少个临时文件, 最后合并
    //      4.2 创建一个大小为 T 的文件, 用文件偏移量直接存储
    let f = crossbeam::scope(|scope| {
        // 生产者线程
        // 生成 n_msgs 个消息
        scope.spawn(|_| {
            for i in 1..=n_msgs {
                snd1.send(i).unwrap();
                println!("Source send: {}", i);
                thread::sleep(Duration::from_millis(100));
            }
            // 发送完毕后丢弃生产者
            drop(snd1);
        });

        // 创建 n_workers 个工作线程
        for _ in 1..=n_workers {
            // 工作线程向生产者返回消息
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());

            // sendr, recvr 移动到闭包
            scope.spawn(move |_| {
                // thread::sleep(Duration::from_millis(500));
                for msg in recvr.iter() {
                    println!("Worker {:?} recevied: {}", thread::current().id(), msg);
                    // 把值处理后, 送回
                    sendr.send(msg * 2).unwrap();
                }
            });
        }
        drop(snd2);
        // 接收
        for msg in rcv2.iter() {
            println!("Sink recived: {}\n", msg);
        }
    });

    f.unwrap()
}
