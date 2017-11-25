
extern crate ws;
extern crate clap;
extern crate term;
extern crate env_logger;

use ws::listen;
use clap::{App, Arg};

fn main() {

    // Setup logging
    env_logger::init().unwrap();

    // Setup cli args
    let matches = App::new("Sonic")
        .version("0.1")
        .author("Noah Rinehart <nrinehar@purdue.edu>, Sanjeet Suhag <suhag@purdue.edu>")
        .about("An encrypted websocket chat app.")
        .arg(Arg::with_name("port")
                .help("The port of the server")
                .required(false)
                .index(1))
        .get_matches();

    let port: String = matches.value_of("port").unwrap_or("3021").to_string();
    let ip: String = "127.0.0.1:".to_string() + &port;

    println!("Starting server on {}", port);
    if let Err(error) = listen(ip, |out| {
        move |msg| {
            println!("Server recieved: '{}'", msg);
            out.send(msg)
        }
    }) {
        println!("Failed to create WebSocket. Error: {:?}", error);
    } 
}

