extern crate ws;
extern crate nickel;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use(bson, doc)] extern crate bson;
extern crate mongodb;
extern crate chrono;

pub mod html_server;
pub mod ws_server;
pub mod db;

use std::thread;
use std::path::Path;

use html_server::*;
use db::*;


use ws::{listen, Sender, Handler, CloseCode};
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult, Options};
use serde_json::{Value};
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use chrono::prelude::{DateTime, Utc};

struct WSServer {
    ws: Sender,
    db: Collection,
}

impl Handler for WSServer {

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        if let Some(ip_addr) = try!(shake.remote_addr()) {
            println!("Connection opened from {}.", ip_addr)
        } else {
            println!("Unable to obtain client's IP address.")
        }
        Ok(())
    }


    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        // TODO: Save message
        let msg_str: String = msg.clone().into_text().unwrap();
        save_to_db(&self.db, &msg_str);

        // echo it back
        self.ws.broadcast(msg)
    }

    fn on_close(&mut self, code: CloseCode, _: &str) {
        println!("Client disconnected with reason '{:?}'", code);
    }

    fn on_error(&mut self, err: ws::Error) {
        println!("Shutting down server for error: {}", err);
        self.ws.shutdown().unwrap();
    }
}


fn main() {
    // Setup HTML thread
    let mut html = Nickel::new();
    html.options = Options::default().output_on_listen(false);
    html.get("/*", html_middleware);
    html.get("/api/:room", api_middleware);

    let html_thread = thread::spawn(move || {
        html.listen("127.0.0.1:3000").expect("Html fail couldn't start!");
    });

    // Setup WebSocket thread
    let ws_ip = "127.0.0.1:3021";
    let ws_thread = thread::spawn(move || {
        listen(ws_ip, |out| {
            let coll = get_database_col();
            WSServer { ws: out, db: coll }
        }).unwrap()
    });

    let _ = html_thread.join();
    let _ = ws_thread.join();
}

