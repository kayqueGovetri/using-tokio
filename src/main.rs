use tokio::time;
use log::Level;
use tokio::io::AsyncReadExt;

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib (n - 2),
    }
}

async fn sleeper(name: &str) {
    log::info!("{}: Sleeping", name);
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("{}: Awake!", name);
}
async fn reader() {
    log::info!("Reading some beeg data");
    let mut f = tokio::fs::File::open("beeg.csv").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read beeg {} bytes!", contents.len());

    tokio::task::spawn_blocking(move || {
        log::info!("computing fib(40)");
        fib(40);
        log::info!("Done computing fib(40)");
    }).await.unwrap();
}

async fn run() {
    tokio::spawn(async {
        sleeper("Floquinho").await
    });
    tokio::join!(
        sleeper("Sherma"),
        reader(),
    );
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();
    log::info!("Took {:?} seconds", end - start);
}