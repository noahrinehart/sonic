use db;

use ws;
use ws::{listen, Sender, Handler, CloseCode, Handshake};
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult, Options};
use serde_json::{Value};
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use chrono::prelude::{DateTime, Utc};

pub struct WSMessage {
    message: String,
    room: String,
    encrypted: bool,
    created_at: DateTime<Utc>,
}

pub struct WSServer {
    pub ws: Sender,
    pub db: Collection,
}

impl Handler for WSServer {

    fn on_open(&mut self, shake: Handshake) -> ws::Result<()> {
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
        db::save_to_db(&self.db, &msg_str);

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
