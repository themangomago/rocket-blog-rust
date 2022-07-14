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

    fn load_user_database(&mut self) {
        self.users.push(User::new(
            "John Doe".to_string(),
            "This is a profile".to_string(),
            "admin".to_string(),
            "admin".to_string(),
        ));
    }

    fn load_post_database(&mut self) {}
}
