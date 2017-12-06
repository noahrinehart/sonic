use db;

use serde_json;
use ws;
use ws::{CloseCode, Handler, Handshake, Sender};
use mongodb::coll::Collection;
use bson::oid::ObjectId;
use chrono::prelude::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSMessage {
    pub _id: Option<ObjectId>,
    pub message: String,
    pub room: String,
    pub encrypted: bool,
    pub created_at: Option<DateTime<Utc>>,
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
        let msg_str: String = msg.clone().into_text().unwrap();
        println!("{}", msg_str);
        let msg_obj: WSMessage = serde_json::from_str(&msg_str).unwrap();
        db::save_to_db(&self.db, msg_obj.clone());

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
