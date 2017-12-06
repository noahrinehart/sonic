
use ws_server::WSMessage;

use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use chrono::prelude::{DateTime, Utc};

pub fn get_database_col() -> Collection {
    let client = Client::connect("localhost", 27017).expect("Failed to connect to database");
    client.db("sonic").collection("messages")
}

pub fn save_to_db(db: &Collection, msg: WSMessage) {
    let time: DateTime<Utc> = Utc::now();
    let created_at = Bson::from(time);
    let doc = doc! {
        "message": msg.message,
        "room": msg.room,
        "encrypted": msg.encrypted,
        "created_at": created_at,
    };
    println!("{:?}", doc);
    db.insert_one(doc.clone(), None).ok().expect("Failed to insert document");
}

// pub fn fetch_messages_from_room(db: &Collection, room: &str) -> Vec<WSMessage> {
//     let cursor = db.find(doc! { }, None).unwrap();
//     Vec::new()
// }
