#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::path::{Path, PathBuf};

use rocket::{
    http::Cookies,
    request::FlashMessage,
    response::{NamedFile, Redirect},
    Data, State,
};
use rocket_contrib::templates::Template;
use serde::Serialize;
use tera::Context;

mod blog;
mod database;
mod user;

use database::database_model::StateHandler;

#[derive(Serialize)]
pub struct FlashNotification<'a> {
    pub severity: &'a str, // Name
    pub message: &'a str,  // Msg
}

// Default entrypoint
#[get("/")]
fn index(
    flash: Option<FlashMessage>,
    mut cookies: Cookies,
    database: State<StateHandler>,
) -> Template {
    blog::index(flash, cookies, database)
}

// Deliever assets
#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

// Checks if user is logged in and provides user data to html context
pub fn add_user_cookie_to_context(mut cookies: Cookies, context: &mut Context) {
    if let Some(user) = cookies.get_private("user") {
        println!("get_user_cookie: {}", user.value());
        context.insert("user", user.value());
    }
    if let Some(admin) = cookies.get_private("admin") {
        println!("get_user_cookie: {}", admin.value());
        context.insert("admin", admin.value());
    }
}

// Checks for flash messages and provides flash data to html context
pub fn add_flash_messages_to_context(flash: Option<FlashMessage>, context: &mut Context) {
    println!("add_flash_messages_to_context");
    if flash.is_some() {
        let flash = flash.unwrap();
        println!("get_flash_message: {}", flash.msg());
        let message = FlashNotification {
            severity: &flash.name(),
            message: &flash.msg(),
        };
        context.insert("flash", &message);
    }
}

// Setup rocket
fn build_rocket(db: StateHandler) -> rocket::Rocket {
    rocket::ignite()
        .manage(db)
        .mount("/", routes![index, files])
        .mount("/user", user::get_routes())
        .mount("/posts", blog::get_routes())
        .attach(Template::fairing())
}

fn main() {
    let mut database = StateHandler::new();
    database.load_databases();

    build_rocket(database).launch();
}
