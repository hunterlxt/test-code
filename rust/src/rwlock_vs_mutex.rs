use std::sync::atomic;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

const ROUND: usize = 10000000;

#[allow(dead_code)]
pub fn run_this_test() {
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

            tx.send(v).unwrap();
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
