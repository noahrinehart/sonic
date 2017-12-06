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

use ws_server::WSServer;

use ws::{listen, Sender, Handler, CloseCode};
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult, Options};
use serde_json::{Value};
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use chrono::prelude::{DateTime, Utc};



fn main() {
    // Setup HTML thread
    let html = html_server::new_server();

    let html_thread = thread::spawn(move || {
        html.listen("127.0.0.1:3000").expect("Html fail couldn't start!");
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

