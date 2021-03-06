use std::{
    fs::File,
    io::{BufReader, BufWriter},
    sync::Mutex,
};

extern crate serde_json;
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

    pub fn get_post(&self, uuid: String) -> Option<Post> {
        for post in self.posts.lock().unwrap().iter() {
            if post.uuid == uuid {
                return Some(post.clone());
            }
        }
        None
    }

    pub fn get_post_id_by_uuid(&self, uuid: String) -> Option<usize> {
        for (i, post) in self.posts.lock().unwrap().iter().enumerate() {
            if post.uuid == uuid {
                return Some(i);
            }
        }
        None
    }

    pub fn get_post_by_id(&self, id: usize) -> Option<Post> {
        self.posts.lock().unwrap().get(id).cloned()
    }

    pub fn get_post_by_uuid(&self, uuid: String) -> Option<Post> {
        for post in self.posts.lock().unwrap().iter() {
            if post.uuid == uuid {
                return Some(post.clone());
            }
        }
        None
    }

    pub fn get_user_id_by_username(&self, username: String) -> Option<usize> {
        for (i, user) in self.users.lock().unwrap().iter().enumerate() {
            if user.credentials.username == username {
                return Some(i);
            }
        }
        None
    }

    pub fn get_user_by_id(&self, id: usize) -> Option<User> {
        self.users.lock().unwrap().get(id).cloned()
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

    pub fn get_posts_by_user(&self, username: &str) -> Vec<Post> {
        let mut posts = Vec::new();
        for post in self.posts.lock().unwrap().iter() {
            if post.author == username {
                posts.push(post.clone());
                if posts.len() == 5 {
                    break;
                }
            }
        }
        posts
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
