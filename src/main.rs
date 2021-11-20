use signal_service::SignalServiceWrapper;
use tokio::sync::mpsc;

mod config;
mod error;
mod service;
mod signal_service;
mod store;

#[tokio::main]
async fn main() {
    println!("starting crayfish {}", env!("GIT_HASH"));
    let (tx, rx) = mpsc::channel(1);

    tokio::task::spawn(service::start_websocket(tx));

    let signal_service = SignalServiceWrapper::new(rx);
    signal_service.run().await;
}
