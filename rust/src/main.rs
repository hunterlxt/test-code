// use fail::fail_point;

// fn say_hello() {
//     let a = 0;
//     let b = 1;
//     fail_point!("before_print", a != b, |_| { println!("hi~") });
//     println!("Hello World~");
// }

// fn main() {
//     say_hello();
//     fail::cfg("before_print", "return");
//     say_hello();
//     say_hello();
//     fail::remove("before_print");
//     say_hello();
// }

use std::sync::atomic;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

const ROUND: usize = 10000000;

fn main() {
    let start = std::time::Instant::now();
    test_mutex();
    println!("mutex: {}", start.elapsed().as_secs_f64());

    let start = std::time::Instant::now();
    test_rwlock();
    println!("rwlock: {}", start.elapsed().as_secs_f64());

    let start = std::time::Instant::now();
    test_atomic();
    println!("atomic: {}", start.elapsed().as_secs_f64());

    let start = std::time::Instant::now();
    test_atomic_ch();
    println!("local and channel: {}", start.elapsed().as_secs_f64());
}

fn test_mutex() {
    let mut handlers = vec![];
    let var = Arc::new(Mutex::new(1 as u64));
    for _ in 0..8 {
        let v = var.clone();
        let h = std::thread::spawn(move || {
            for _ in 0..ROUND {
                let mut var = v.lock().unwrap();
                *var += 1;
            }
        });
        handlers.push(h);
    }
    for h in handlers {
        h.join().unwrap();
    }
    println!("Finish: {}", var.lock().unwrap());
}

fn test_rwlock() {
    let mut handlers = vec![];
    let var = Arc::new(RwLock::new(1 as u64));
    for _ in 0..8 {
        let v = var.clone();
        let h = std::thread::spawn(move || {
            for _ in 0..ROUND {
                let mut var = v.write().unwrap();
                *var += 1;
            }
        });
        handlers.push(h);
    }
    for h in handlers {
        h.join().unwrap();
    }
    println!("Finish: {}", var.read().unwrap());
}

fn test_atomic() {
    let mut handlers = vec![];
    let var = Arc::new(atomic::AtomicU64::new(1));
    for _ in 0..8 {
        let v = var.clone();
        let h = std::thread::spawn(move || {
            for _ in 0..ROUND {
                v.fetch_add(1, atomic::Ordering::Relaxed);
            }
        });
        handlers.push(h);
    }
    for h in handlers {
        h.join().unwrap();
    }
    println!("Finish: {}", var.load(atomic::Ordering::Relaxed));
}

fn test_atomic_ch() {
    let mut handlers = vec![];

    let (tx, rx) = std::sync::mpsc::channel();
    for _ in 0..8 {
        let tx = tx.clone();
        let h = std::thread::spawn(move || {
            let mut v: u64 = 1;

            for _ in 0..ROUND {
                v += 1;
            }

            tx.send(v);
        });
        handlers.push(h);
    }

    let mut sum = 0;
    for h in handlers {
        h.join().unwrap();
        sum += rx.recv().unwrap();
    }
    println!("Finish: {}", sum);
}

struct Data {
    name: String,
}
