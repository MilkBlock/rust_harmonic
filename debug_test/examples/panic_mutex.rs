use std::{thread, time::Duration};

use parking_lot::{deadlock, Mutex};

fn main() {
    { // only for #[cfg]

        // Create a background thread which checks for deadlocks every 10s
        let a = thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                let deadlocks = deadlock::check_deadlock();
                if deadlocks.is_empty() {
                    continue;
                }

                println!("{} deadlocks detected", deadlocks.len());
                for (i, threads) in deadlocks.iter().enumerate() {
                    println!("Deadlock #{}", i);
                    for t in threads {
                        println!("Thread Id {:#?}", t.thread_id());
                        println!("{:#?}", t.backtrace());
                    }
                }
            }
        });
    } // only for #[cfg]

    let mutex = Mutex::new(42);

    // 第一次加锁成功
    let mut data = mutex.lock();
    println!("First lock: {}", *data);

    // 同一线程内尝试第二次加锁（递归加锁）
    let _data2 = mutex.lock(); // 这里会 panic！
    println!("This line will never be reached.");
}