use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub profile: String,
    pub credentials: UserCredentials,
}

#[derive(Serialize, Deserialize)]
pub struct UserCredentials {
    pub username: String,
    password_hash: String,
}

impl UserCredentials {
    pub fn check_password(&self, password: &str) -> bool {
        let mut sha512 = Sha512::new();
        sha512.update(password);
        let password_hash = hex::encode(sha512.finalize().as_slice());
        self.password_hash == password_hash
    }
}

impl User {
    pub fn new(name: String, profile: String, username: String, password_hash: String) -> User {
        User {
            name,
            profile,
            credentials: UserCredentials {
                username: username,
                password_hash: password_hash,
            },
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_profile(&self) -> String {
        self.profile.clone()
    }
}
