pub mod actor1 {
    use crate::example::actor2::Actor2Message;
    use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

    #[derive(Debug)]
    pub enum Actor1Message {
        Ping,
        Count(tokio::sync::oneshot::Sender<usize>),
    }

    pub struct Actor1 {
        channel: UnboundedReceiver<Actor1Message>,
        actor2_ref: UnboundedSender<Actor2Message>,
        ping_counter: usize,
    }

    impl Actor1 {
        pub fn run(channel: UnboundedReceiver<Actor1Message>, actor2_ref: UnboundedSender<Actor2Message>) {
            let mut actor1 = Actor1 {
                channel,
                actor2_ref,
                ping_counter: 0,
            };

            tokio::spawn(async move {
                loop {
                    let message = actor1.channel.recv().await.unwrap();
                    match message {
                        Actor1Message::Ping => {
                            println!("Actor1 received: Ping");
                            actor1.ping_counter += 1;
                            actor1.actor2_ref.send(Actor2Message::Pong).unwrap();
                        }
                        Actor1Message::Count(sender) => {
                            println!("Actor1 received: Count");
                            sender.send(actor1.ping_counter).unwrap();
                        }
                    }
                }
            });
        }
    }
}

pub mod actor2 {
    use crate::example::actor1::Actor1Message;
    use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

    #[derive(Debug)]
    pub enum Actor2Message {
        Pong,
    }

    pub struct Actor2 {
        channel: UnboundedReceiver<Actor2Message>,
        actor1_ref: UnboundedSender<Actor1Message>,
    }


    impl Actor2 {
        pub fn run(channel: UnboundedReceiver<Actor2Message>, actor1_ref: UnboundedSender<Actor1Message>) {
            let mut actor2 = Actor2 {
                channel,
                actor1_ref,
            };


            tokio::spawn(async move {
                loop {
                    let message = actor2.channel.recv().await.unwrap();
                    match message {
                        Actor2Message::Pong => {
                            println!("Actor2 received: Pong");
                            actor2.actor1_ref.send(Actor1Message::Ping).unwrap();
                        }
                    }
                }
            });
        }
    }
}