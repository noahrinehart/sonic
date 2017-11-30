extern crate ws;
extern crate env_logger;
extern crate iron;
extern crate staticfile;
extern crate router;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::thread;
use std::path::Path;

use ws::{listen, Message, Sender, Handler, CloseCode};
use iron::Iron;
use staticfile::Static;
use router::Router;
use serde_json::{Value};

struct Server {
    ws: Sender,
}

impl Handler for Server {

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        if let Some(ip_addr) = try!(shake.remote_addr()) {
            println!("Connection opened from {}.", ip_addr)
        } else {
            println!("Unable to obtain client's IP address.")
        }
        Ok(())
    }


    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let replaced_msg = msg.clone().into_text().unwrap().replace("\n", "");
        println!("Server got message '{}'. ", replaced_msg);
        // TODO: Save message
        let message: String = msg.clone().into_text().unwrap();
        
        let data = r#"{
            "name": "John Doe",
            "age": 43,
            "phones": [
              "+44 1234567",
              "+44 2345678"
            ]
          }"#;
        let v: Value = serde_json::from_str(data).expect("Couldn't parse message!");

        println!("{}", v["name"]);
        // echo it back
        self.ws.broadcast(msg)
    }

    fn on_close(&mut self, code: CloseCode, _: &str) {
        println!("Client disconnected with reason '{:?}'", code);
        self.ws.broadcast("Someone has left the room!").unwrap();
    }

    fn on_error(&mut self, err: ws::Error) {
        println!("Shutting down server for error: {}", err);
        self.ws.shutdown().unwrap();
    }
}


fn main() {

    // Setup logging
    env_logger::init().unwrap();

    // Setup HTML server
    let mut router = Router::new();
    router.get("/", Static::new(Path::new("public/index.html")), "index");
    router.get("/:room", Static::new(Path::new("public/index.html")), "room");
    let html_server = thread::spawn(move || {
        Iron::new(router).http("127.0.0.1:3000").unwrap();
    });

    // Setup WebSocket server
    let ws_ip = "127.0.0.1:3021";
    let ws_server = thread::spawn(move || {
        listen(ws_ip, |out| {
            Server { ws: out }
        }).unwrap()
    });


    let _ = html_server.join();
    let _ = ws_server.join();
}

