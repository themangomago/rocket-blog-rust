use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;
use tera::Context;

use crate::database::database_model::StateHandler;
use crate::{database, get_user_cookie};

#[path = "user_model.rs"]
pub mod user_model;

////////////////////////////////////////////////////////////////////////////////
/// Routes
////////////////////////////////////////////////////////////////////////////////

#[derive(FromForm)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[get("/login")]
fn login(cookies: Cookies) -> Template {
    println!("login");
    let mut context = Context::new();
    get_user_cookie(cookies, &mut context);
    Template::render("user/login", &context.into_json())
}

#[post("/login", data = "<form>")]
fn login_post(mut cookies: Cookies, form: Form<Login>, database: State<StateHandler>) -> Redirect {
    let user = database.get_user(&form.username);

    if !user.is_none() {
        let user = user.unwrap();
        if user.credentials.check_password(&form.password) {
            // User found - set cookie
            let mut cookie = Cookie::new("user", form.username.clone());
            cookie.set_path("/");
            cookies.add_private(cookie);
            return Redirect::to("/");
        } else {
            // User found - wrong password
            // TODO: display error message
            println!("Error: Wrong password");
            return Redirect::to("/user/login");
        }
    } else {
        // User not found
        // TODO: display error message
        println!("Error: User not found");
        return Redirect::to("/user/login");
    }
}

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user"));
    Redirect::to("/")
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![login, login_post, logout]
}
