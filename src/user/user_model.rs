use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FlashMessage, Form, FromRequest, Request, State};
use rocket::response::{Flash, Redirect};
use rocket::Outcome;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub admin_rights: u8,
    pub profile: UserProfile,
    pub credentials: UserCredentials,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub bio: String,
    pub twitter: String,
    pub github: String,
}

pub struct AuthenticatedUser {
    pub username: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthenticatedUser, Self::Error> {
        let mut cookies = request.cookies();
        if let Some(user) = cookies.get_private("user") {
            return Success(AuthenticatedUser {
                username: user.value().to_string(),
            });
        }
        Failure((Status::raw(401), ()))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserCredentials {
    pub username: String,
    pub password_hash: String,
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
    pub fn new(
        name: String,
        admin_rights: u8,
        user_bio: String,
        user_twitter: String,
        user_github: String,
        username: String,
        password_hash: String,
    ) -> User {
        User {
            name,
            profile: UserProfile {
                bio: user_bio,
                twitter: user_twitter,
                github: user_github,
            },
            admin_rights,
            credentials: UserCredentials {
                username: username,
                password_hash: password_hash,
            },
        }
    }

    pub fn get_user(&self, username: &str) -> Option<User> {
        if self.credentials.username == username {
            Some(self.clone())
        } else {
            None
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
