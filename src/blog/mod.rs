use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;
use tera::Context;

use crate::database::model::StateHandler;
use crate::{database, get_user_cookie};

#[path = "model.rs"]
pub mod model;

////////////////////////////////////////////////////////////////////////////////
/// Routes
////////////////////////////////////////////////////////////////////////////////

#[get("/create")]
fn create(cookies: Cookies) -> Template {
    let mut context = Context::new();
    get_user_cookie(cookies, &mut context);
    Template::render("posts/create", &context.into_json())
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![create]
}
