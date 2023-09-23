mod error;
mod message;
mod server;

use std::{convert::TryFrom, io};

use serde_json::Deserializer;
use server::Server;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

use crate::message::Message;

fn main() -> color_eyre::Result<()> {
    initialise_tracing();

    let stdin = io::stdin();
    info!("Got stdin");

    let mut buf = String::new();
    stdin.read_line(&mut buf)?;

    let initial_request: Message = serde_json::from_str(&buf)?;
    info!(target: "Received request", request = ?initial_request);

    let mut server = Server::try_from(&initial_request)?;
    info!("Initialised server");

    let initial_response = server.handle_initial_request(initial_request)?;

    println!("{}", &initial_response);
    info!(target: "Sent response", response = ?initial_response);

    let requests = Deserializer::from_reader(stdin).into_iter::<Message>();
    requests.flatten().enumerate().for_each(|(index, request)| {
        info!(target: "Received request", request = ?request);
        if let Ok(Some(response)) = server.handle_request(request) {
            println!("{}", &response);
            info!(target: "Sent response", response = ?response);
        };

        if index % 4 == 0 {
            if let Some(messages) = server.generate_gossip() {
                messages.iter().for_each(|message| {
                    println!("{}", &message);
                    info!(target: "Sent request", request = ?message);
                })
            }
        };
    });

    if let Some(messages) = server.generate_gossip() {
        messages.iter().for_each(|message| {
            println!("{}", &message);
            info!(target: "Sent request", request = ?message);
        })
    }
    Ok(())
}

fn initialise_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .with_writer(io::stderr)
        .init();
}
