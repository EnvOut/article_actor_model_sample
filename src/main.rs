use std::time::Duration;
use tokio::sync::mpsc::unbounded_channel;
use tokio::time::sleep;

pub mod example;

#[tokio::main]
async fn main() {
    // Initialize channels
    let (actor1_tx, actor1_rx) = unbounded_channel::<example::actor1::Actor1Message>();
    let (actor2_tx, actor2_rx) = unbounded_channel::<example::actor2::Actor2Message>();

    // Init actors
    example::actor1::Actor1::run(actor1_rx, actor2_tx);
    example::actor2::Actor2::run(actor2_rx, actor1_tx.clone());

    // Send ping message to the actor1
    actor1_tx.send(example::actor1::Actor1Message::Ping).unwrap();

    // sleep for a while to let the actors do their job (10 milliseconds)
    sleep(Duration::from_millis(10)).await;

    // ask actor1 to return the ping count
    let (tx, rx) = tokio::sync::oneshot::channel();
    actor1_tx.send(example::actor1::Actor1Message::Count(tx)).unwrap();
    let ping_count = rx.await.unwrap();
    println!("Ping count: {}", ping_count);
}