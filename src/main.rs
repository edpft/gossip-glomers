mod error;
mod message;
mod server;

use std::{
    convert::TryFrom,
    io,
    sync::mpsc::{self, Receiver},
    thread,
};

use serde_json::Deserializer;
use server::Server;

use crate::message::Message;

fn main() -> color_eyre::Result<()> {
    let stdin = io::stdin();
    eprintln!("[INFO] - Got stdin");

    let mut buf = String::new();
    stdin.read_line(&mut buf)?;

    let initial_request: Message = serde_json::from_str(&buf)?;
    eprintln!(
        "[INFO] - Received initialisation request: {}",
        &initial_request
    );

    let mut server = Server::try_from(&initial_request)?;
    eprintln!("[INFO] - Initialised server");

    let initial_response = server.handle_initial_request(initial_request)?;

    println!("{}", &initial_response);
    eprintln!("[INFO] - Sent initialisation response: {initial_response}");

    let requests = Deserializer::from_reader(stdin).into_iter::<Message>();
    requests.flatten().enumerate().for_each(|(index, request)| {
        eprintln!("[INFO] - Received request: {}", &request);
        if let Some(response) = server.handle_request(request) {
            println!("{}", &response);
            eprintln!("[INFO] - Sent response: {}", response);
        };

        if index % 4 == 0 {
            if let Some(messages) = server.generate_gossip() {
                messages.iter().for_each(|message| {
                    println!("{}", &message);
                    eprintln!("[INFO] - Sent request: {}", message);
                })
            }
        };
    });

    if let Some(messages) = server.generate_gossip() {
        messages.iter().for_each(|message| {
            println!("{}", &message);
            eprintln!("[INFO] - Sent request: {}", message);
        })
    }
    Ok(())
}
