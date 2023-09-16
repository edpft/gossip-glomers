mod error;
mod request;
mod response;
mod server;

use request::Request;
use server::Server;

fn main() -> color_eyre::Result<()> {
    let initial_request = Request::from_stdin()?;
    let mut server = Server::from_initial_request(&initial_request)?;
    let initial_response = server.handle_initial_request(initial_request)?;
    println!("{}", initial_response);
    loop {
        let request = Request::from_stdin()?;
        let response = server.handle_request(request)?;
        println!("{}", response);
    }
}
