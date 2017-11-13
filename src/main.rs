
extern crate ws;
extern crate env_logger;

use ws::listen;

static IP: &'static str = "127.0.0.1:3012";

fn main() {

    env_logger::init().unwrap();

    if let Err(error) = listen(IP, |out| {
        move |msg| {
            println!("Server recieved: '{}'", msg);
            out.send(msg)
        }
    }) {
        println!("Failed to create WebSocket. Error: {:?}", error);
    }
}
