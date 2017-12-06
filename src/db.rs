
use ws_server::WSMessage;

use ws::{listen, Sender, Handler, CloseCode};
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult, Options};
use serde_json;
use serde_json::{Value};
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use chrono::prelude::{DateTime, Utc};



pub fn get_database_col() -> Collection {
    let client = Client::connect("localhost", 27017).expect("Failed to connect to database");
    client.db("sonic").collection("messages")
}

pub fn save_to_db(db: &Collection, msg: &str) {
    let msg_json: Value = serde_json::from_str(msg).unwrap();
    let message = msg_json["message"].as_str().unwrap();
    let room = msg_json["room"].as_str().unwrap();
    let encrypted = msg_json["encrypted"].as_bool().unwrap();
    let time: DateTime<Utc> = Utc::now();
    let created_at = Bson::from(time);
    let doc = doc! {
        "message": message,
        "room": room,
        "encrypted": encrypted,
        "created_at": created_at,
    };
    println!("{:?}", doc);
    db.insert_one(doc.clone(), None).ok().expect("Failed to insert document");
}

pub fn fetch_messages_from_room(db: &Collection, room: &str) -> Vec<WSMessage> {
    Vec::new()
}