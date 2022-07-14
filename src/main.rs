#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

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
        .mount("/", routes![index])
        .attach(Template::fairing())
}

fn main() {
    build_rocket().launch();
}
