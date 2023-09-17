mod error;
mod request;
mod response;
mod server;

use std::{
    convert::TryFrom,
    io::{stdin, stdout, Write},
};

use request::Request;
use serde_json::Deserializer;
use server::Server;

fn main() -> color_eyre::Result<()> {
    let stdin_lock = stdin().lock();
    let mut stdout_lock = stdout().lock();

    let mut requests = Deserializer::from_reader(stdin_lock).into_iter::<Request>();

    let initial_request = requests
        .next()
        .expect("Received init message")
        .expect("Deserialized request from init message");
    let mut server = Server::try_from(&initial_request)?;
    let initial_response = server.handle_initial_request(initial_request)?;
    writeln!(stdout_lock, "{}", initial_response)?;

    for request in requests.flatten() {
        let response = server.handle_request(request)?;
        writeln!(stdout_lock, "{}", response)?;
    }
    Ok(())
}
