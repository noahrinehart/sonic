use chrono::prelude::{DateTime, Utc};


pub struct WSMessage {
    message: String,
    room: String,
    encrypted: bool,
    created_at: DateTime<Utc>,
}