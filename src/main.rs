mod error;
mod request;
mod response;
mod server;

use std::io::{stdin, stdout, BufRead, Write};

use error::Error;
use request::Request;
use server::Server;

fn main() -> color_eyre::Result<()> {
    let stdin_lock = stdin().lock();
    let mut stdout_lock = stdout().lock();

    let mut lines = stdin_lock.lines().flatten();

    let mut server = if let Some(initial_request_string) = lines.next() {
        let initial_request = Request::from_string(initial_request_string)?;
        let mut server = Server::from_initial_request(&initial_request)?;
        let initial_response = server.handle_initial_request(initial_request)?;
        writeln!(stdout_lock, "{}", initial_response)?;
        Ok(server)
    } else {
        let error = Error::Initialisation;
        Err(error)
    }?;

    for line in lines {
        let request = Request::from_string(line)?;
        let response = server.handle_request(request)?;
        writeln!(stdout_lock, "{}", response)?;
    }
    Ok(())
}
