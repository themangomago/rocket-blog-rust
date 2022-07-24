use std::{
    fs::File,
    io::{BufReader, BufWriter},
    ops::{Deref, DerefMut},
    sync::Mutex,
};

extern crate serde_json;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use uuid::Uuid;

use crate::{blog::post_model::Post, user::user_model::User};

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

    pub fn get_posts(&self) -> Vec<Post> {
        self.posts.lock().unwrap().clone()
    }

    pub fn create_post(&mut self, post: Post) {
        //TODO: this aint no work :(
        self.posts.lock().unwrap().push(post);
    }

    pub fn get_user(&self, username: &str) -> Option<User> {
        for user in self.users.lock().unwrap().iter() {
            if user.credentials.username == username {
                //name: String, profile: String, username: String, password_hash: String
                let user: User = User::new(
                    user.name.clone(),
                    user.admin_rights,
                    user.profile.bio.clone(),
                    user.profile.twitter.clone(),
                    user.profile.github.clone(),
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
                1,
                "Admin account, please delete me after adding real users".to_string(),
                "themangomago".to_string(),
                "themangomago".to_string(),
                "admin".to_string(),
                password_hash,
            );
            self.users = Mutex::new(vec![dummy_user]);
            self.save_user_database();
            println!("User database created with dummy user: admin pw: admin");
        }
    }

    pub fn save_user_database(&self) {
        let file = File::create("database/users.json").unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self.users).unwrap();
    }

    fn load_post_database(&mut self) {
        let path = "database/posts.json";
        if std::path::Path::new(path).exists() {
            // Post database found - load it
            let file = File::open(path).unwrap();
            let mut reader = BufReader::new(file);
            self.posts = serde_json::from_reader(&mut reader).unwrap();
        } else {
            // Post database not found - create one
            let dummy_post: Post = Post::new(
                Uuid::new_v4().to_string(),
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                "admin".to_string(),
                "Dummy post".to_string(),
                "This is a dummy post. Congratulations! Be sure to add some real content to it."
                    .to_string(),
            );
            self.posts = Mutex::new(vec![dummy_post]);

            self.save_post_database();
        }
    }

    pub fn save_post_database(&self) {
        let file = File::create("database/posts.json").unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self.posts).unwrap();
    }
}
