pub struct User {
    pub name: String,
    pub profile: String,
    pub credentials: UserCredentials,
}

pub struct UserCredentials {
    pub username: String,
    password_hash: String,
}

impl UserCredentials {
    pub fn check_password(&self, password: &str) -> bool {
        // TODO: Hash user password and compare to password_hash
        self.password_hash == password
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
