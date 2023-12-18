mod counter;
mod node;
mod payload;

use std::{
    io,
    time::{Duration, Instant},
};

use maelstrom_protocol::{gossip::Gossip, messages::Message, nodes::Node};
use serde_json::Deserializer;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

use crate::{node::CounterNode, payload::CounterPayload};

fn main() -> color_eyre::Result<()> {
    initialise_tracing();
    let mut node = CounterNode::new();

    let stdin = io::stdin();
    info!("Got stdin");

    let requests = Deserializer::from_reader(stdin).into_iter::<Message<CounterPayload>>();

    let duration = Duration::from_millis(95);
    let mut now = Instant::now();

    for request in requests.flatten() {
        node = node.handle(request);
        if now.elapsed() > duration {
            node.gossip();
            now = Instant::now();
        }
    }
    Ok(())
}

fn initialise_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .with_writer(io::stderr)
        .with_ansi(false)
        .init();
}
