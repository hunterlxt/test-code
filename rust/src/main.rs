use futures::channel::mpsc;
use futures::executor::block_on;
use futures::future::{Either, Future, FutureExt, Shared, TryFuture, TryFutureExt};
use futures::sink::{Sink, SinkExt};
use futures::stream::{Stream, StreamExt};
use futures::task::{Context, Poll};
use tokio::runtime;
use tokio::time;

use std::collections::VecDeque;
use std::pin::Pin;
use std::thread;
use std::time as stdtime;

fn main() {
    request(|s| Box::new(s));
}

fn get_guard(m: &std::sync::Mutex<i32>) -> std::sync::MutexGuard<i32> {
    let data = m.lock().unwrap();
    data
}

fn get_future(flag: bool) -> Box<dyn Future<Output = ()>> {
    if flag {
        Box::new(async move {
            println!("true");
        })
    } else {
        Box::new(async move {
            println!("false");
        })
    }
}

fn request<F>(mut exec: F)
where
    F: FnMut(String) -> Box<String> + 'static,
{
    let data = exec("abc".to_owned());
    println!("{}", *data);
}
