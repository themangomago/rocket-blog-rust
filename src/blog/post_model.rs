use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub uuid: String,
    pub author_username: String,
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn new(uuid: String, author_username: String, title: String, body: String) -> Post {
        Post {
            uuid,
            author_username,
            title,
            body,
        }
    }
}
