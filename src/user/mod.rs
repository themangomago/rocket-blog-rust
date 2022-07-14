use rocket_contrib::templates::Template;
use tera::Context;

#[path = "model.rs"]
pub mod model;

////////////////////////////////////////////////////////////////////////////////
/// Routes
////////////////////////////////////////////////////////////////////////////////

#[get("/login")]
fn login() -> Template {
    let mut context = Context::new();
    Template::render("user/login", &context.into_json())
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![login]
}
