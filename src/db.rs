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
    db.insert_one(doc.clone(), None)
        .ok()
        .expect("Failed to insert document");
}

pub fn fetch_messages_from_room(db: &Collection, room: &str) -> Vec<WSMessage> {
    let cursor = db.find(Some(doc! {"room": format!("/{}", room)}), None)
        .expect("Error accessing database");
    let mut messages = Vec::new();
    for doc in cursor {
        let doc_expect = doc.expect("Failed to get doc");
        messages.push(WSMessage {
            _id: Some(
                doc_expect
                    .get_object_id("_id")
                    .expect("No _id key found in record")
                    .to_owned(),
            ),
            message: doc_expect
                .get_str("message")
                .expect("No message key found in record")
                .to_owned(),
            room: doc_expect
                .get_str("room")
                .expect("No room key found in record")
                .to_owned(),
            encrypted: doc_expect
                .get_bool("encrypted")
                .expect("No encrypted key found in record")
                .to_owned(),
            created_at: Some(
                doc_expect
                    .get_utc_datetime("created_at")
                    .expect("No created_at key found in record")
                    .to_owned(),
            ),
        });
    }
    messages
}
