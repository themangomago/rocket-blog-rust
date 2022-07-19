#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::path::{Path, PathBuf};

use rocket::{
    http::Cookies,
    response::{NamedFile, Redirect},
    Data, State,
};
use rocket_contrib::templates::Template;
use tera::Context;

mod blog;
mod database;
mod user;

use database::database_model::StateHandler;

// Default entrypoint
#[get("/")]
fn index(mut cookies: Cookies, database: State<StateHandler>) -> Template {
    blog::index(cookies, database)
}

// Deliever assets
#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

// Checks if user is logged in and provides user data to html context
pub fn get_user_cookie(mut cookies: Cookies, context: &mut Context) {
    if let Some(user) = cookies.get_private("user") {
        println!("get_user_cookie: {}", user.value());
        context.insert("user", user.value());
    }
    if let Some(admin) = cookies.get_private("admin") {
        println!("get_user_cookie: {}", admin.value());
        context.insert("admin", admin.value());
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
