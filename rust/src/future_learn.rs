use futures::executor::block_on;

/********************************************************/

async fn hello_world() {
    println!("hello, world!");
}

pub fn run_hello_world() {
    // f 仅仅是一个 Future 需要在 executor 上才能执行
    let f = hello_world();
    block_on(f);
}

/********************************************************/

async fn learn_song() -> String {
    println!("learning stars");
    std::thread::sleep(std::time::Duration::from_secs(1));
    "stars".to_owned()
}
async fn sing_song(song: String) {
    println!("sing {}", song);
    std::thread::sleep(std::time::Duration::from_secs(1));
}
async fn dance() -> u64 {
    let time = 2;
    println!("dancing {} s", time);
    std::thread::sleep(std::time::Duration::from_secs(1));
    std::thread::sleep(std::time::Duration::from_secs(1));
    time
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_learn_sing_dance() {
    let f1 = learn_and_sing();
    let f2 = dance();
    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    let ret = futures::join!(f1, f2);
    println!("{:?}", ret);
}

pub fn run_sing_song_test() {
    block_on(async_learn_sing_dance());
}

/********************************************************/
