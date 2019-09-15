#![warn(rust_2018_idioms)]

use std::{collections::HashMap, error::Error};

use tokio::{
    self,
    sync::{Lock},
    runtime::Runtime,
};

fn main() {
    // we use a runtime, so we can specifically wait on all futures
    // using Runtime::shutdown_on_idle
    let rt = Runtime::new().unwrap();

    // construct the shared state that each task will use
    let state = Lock::new(Shared::new());

    // create our community of peers
    let peers = (1..10).into_iter().map(|i| Peer { id: format!("peer{}", i)}).collect::<Vec<Peer>>();

    rt.block_on(async move {
        for peer in peers {
            let state = state.clone();
            let _ = peer.run(state).await;
        }
    });
}

struct Peer {
    id: String
}

impl Peer {
    async fn run(&self, mut state: Lock<Shared>) -> Result<(), Box<dyn Error>> {
        let mut state = state.lock().await;
        match state.mailbox.get(&self.id) {
            Some(msgs) => println!("{}: I have {} messages!", &self.id, msgs.len()),
            _ => { println!("{}: No messages for me", &self.id) }
        };
        for i in 1..10 {
            let peer = format!("peer{}", i);
            let msgs = state.mailbox.entry(peer).or_insert(vec![]);
            msgs.push(format!("hello from {}", &self.id));
        }
        Ok(())
    }
}

/// Data that is shared between all peers
///
/// This is just a simple mailbox of recipients and messages
struct Shared {
    mailbox: HashMap<String, Vec<String>>,
}

impl Shared {
    fn new() -> Self {
        Shared {
            mailbox: HashMap::new()
        }

    }
}

