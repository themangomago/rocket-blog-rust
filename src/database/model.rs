use std::{
    fs::File,
    io::{BufReader, BufWriter},
    sync::Mutex,
};

extern crate serde_json;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

use crate::blog::model::Post;
use crate::user::model::User;

pub struct StateHandler {
    pub users: Mutex<Vec<User>>,
    pub posts: Mutex<Vec<Post>>,
}

impl StateHandler {
    pub fn new() -> StateHandler {
        StateHandler {
            users: Mutex::new(Vec::new()),
            posts: Mutex::new(Vec::new()),
        }
    }

    pub fn load_databases(&mut self) {
        self.load_user_database();
        self.load_post_database();
    }

    pub fn get_user(&self, username: &str) -> Option<User> {
        for user in self.users.lock().unwrap().iter() {
            if user.credentials.username == username {
                //name: String, profile: String, username: String, password_hash: String
                let user: User = User::new(
                    user.name.clone(),
                    user.profile.clone(),
                    user.admin_rights,
                    user.credentials.username.clone(),
                    user.credentials.password_hash.clone(),
                );
                return Some(user);
            }
        }
        None
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
                1,
                "admin".to_string(),
                password_hash,
            );
            self.users = Mutex::new(vec![dummy_user]);
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
