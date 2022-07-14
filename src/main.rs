#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::path::{Path, PathBuf};

use rocket::{response::NamedFile, Data};
use rocket_contrib::templates::Template;
use tera::Context;

mod blog;
mod database;
mod user;

use database::model::Database;

// Default entrypoint
#[get("/")]
fn index() -> Template {
    let mut context = Context::new();
    context.insert("test", "test2");
    context.insert("posts", &vec!["post1", "post2", "post3"]);
    Template::render("posts/index", &context.into_json())
}

// Deliever assets
#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

// Setup rocket
fn build_rocket(db: Database) -> rocket::Rocket {
    rocket::ignite()
        .manage(db)
        .mount("/", routes![index, files])
        .mount("/user", user::get_routes())
        .attach(Template::fairing())
}

fn main() {
    let mut database = Database::new();
    database.load_databases();

    build_rocket(database).launch();
}
