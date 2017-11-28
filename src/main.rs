extern crate ws;
extern crate env_logger;
extern crate iron;
extern crate staticfile;
extern crate mount;

use std::thread;
use std::path::Path;

use ws::{listen, Message, Sender, Handler, Result, Error, CloseCode};
use iron::Iron;
use staticfile::Static;
use mount::Mount;

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


    fn on_message(&mut self, msg: Message) -> Result<()> {
        let replaced_msg = msg.clone().into_text().unwrap().replace("\n", "");
        println!("Server got message '{}'. ", replaced_msg);

        // echo it back
        self.ws.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, _: &str) {
        println!("Client disconnected with reason '{:?}'", code);
    }

    fn on_error(&mut self, err: Error) {
        println!("Shutting down server for error: {}", err);
        self.ws.shutdown().unwrap();
    }
}


fn main() {

    // Setup logging
    env_logger::init().unwrap();

    // Setup HTML server
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("public/")));
    let html_server = thread::spawn(move || {
        Iron::new(mount).http("127.0.0.1:3000").unwrap();
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

