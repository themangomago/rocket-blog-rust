#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use std::path::{Path, PathBuf};

use rocket::{
    http::Cookies,
    request::FlashMessage,
    response::{Flash, NamedFile, Redirect},
    Request, State,
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
fn index(flash: Option<FlashMessage>, cookies: Cookies, database: State<StateHandler>) -> Template {
    blog::index(0, flash, cookies, database)
}

// Deliever assets
#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

#[catch(401)]
fn not_authorized(_req: &Request) -> Flash<Redirect> {
    return Flash::error(
        Redirect::to("/user/login"),
        "Error: Not authorized to do this action.",
    );
}

#[catch(404)]
fn not_found(_req: &Request) -> Template {
    return Template::render("error/404", Context::new().into_json());
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
        .register(catchers![not_authorized, not_found])
        .attach(Template::fairing())
}

fn main() {
    let mut database = StateHandler::new();
    database.load_databases();

    build_rocket(database).launch();
}
