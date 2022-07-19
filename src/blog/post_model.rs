use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub uuid: String,
    pub date: String,
    pub author: String,
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn new(uuid: String, date: String, author: String, title: String, body: String) -> Post {
        Post {
            uuid,
            date,
            author,
            title,
            body,
        }
    }
}
