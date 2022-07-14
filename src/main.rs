#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::templates::Template;
use tera::Context;

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();
    context.insert("test", "test2");
    context.insert("posts", &vec!["post1", "post2", "post3"]);
    Template::render("posts/index", &context.into_json())
}

fn build_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, files])
        .attach(Template::fairing())
}

#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

fn main() {
    build_rocket().launch();
}
