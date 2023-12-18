mod node;
mod payload;

use std::io;

use maelstrom_protocol::{messages::Message, nodes::Node};
use serde_json::Deserializer;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

use crate::{node::EchoNode, payload::EchoPayload};

fn main() -> color_eyre::Result<()> {
    initialise_tracing();
    let mut node = EchoNode::new();

    let stdin = io::stdin();
    info!("Got stdin");

    let requests = Deserializer::from_reader(stdin).into_iter::<Message<EchoPayload>>();
    for request in requests.flatten() {
        node = node.handle(request);
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