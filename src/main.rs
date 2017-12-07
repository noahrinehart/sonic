#[macro_use(bson, doc)]
extern crate bson;
extern crate chrono;
extern crate mongodb;
#[macro_use]
extern crate nickel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate ws;

pub mod html_server;
pub mod ws_server;
pub mod db;

use std::thread;

use ws_server::WSServer;
use ws::listen;

fn main() {
    // Setup HTML thread
    let html = html_server::new_server();

    let html_thread = thread::spawn(move || {
        html.listen("127.0.0.1:3000")
            .expect("Html server couldn't start!");
    });

    // Setup WebSocket thread
    let ws_ip = "127.0.0.1:3021";
    let ws_thread = thread::spawn(move || {
        listen(ws_ip, |out| {
            let coll = db::get_database_col();
            WSServer { ws: out, db: coll }
        }).unwrap()
    });

    let _ = html_thread.join();
    let _ = ws_thread.join();
}
