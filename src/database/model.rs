use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

extern crate serde_json;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

use crate::blog::model::Post;
use crate::user::model::User;

pub struct Database {
    pub users: Vec<User>,
    pub posts: Vec<Post>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            users: Vec::new(),
            posts: Vec::new(),
        }
    }

    pub fn load_databases(&mut self) {
        self.load_user_database();
        self.load_post_database();
    }

    pub fn get_user(&self, username: &str) -> Option<&User> {
        self.users
            .iter()
            .find(|user| user.credentials.username == username)
    }

    fn load_user_database(&mut self) {
        let path = "database/users.json";
        if std::path::Path::new(path).exists() {
            // User database found - load it
            let file = File::open(path).unwrap();
            let mut reader = BufReader::new(file);
            self.users = serde_json::from_reader(&mut reader).unwrap();
        } else {
            // User database not found - create a dummy user and db
            let mut sha512 = Sha512::new();
            sha512.update("admin".as_bytes());
            let password_hash = hex::encode(sha512.finalize().as_slice());

            let dummy_user = User::new(
                "John Doe".to_string(),
                "This is a dummy user".to_string(),
                "admin".to_string(),
                password_hash,
            );
            self.users.push(dummy_user);
            self.save_user_database();
            println!("User database created with dummy user: admin pw: admin");
        }
    }

    fn save_user_database(&mut self) {
        let file = File::create("database/users.json").unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self.users).unwrap();
    }

    fn load_post_database(&mut self) {}
}
