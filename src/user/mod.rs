use rocket::http::{Cookie, Cookies};
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_contrib::templates::Template;
use tera::Context;

use crate::database::database_model::StateHandler;
use crate::{
    add_flash_messages_to_context, add_user_cookie_to_context, database, FlashNotification,
};

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
fn login(flash: Option<FlashMessage>, cookies: Cookies) -> Template {
    let mut context = Context::new();
    add_user_cookie_to_context(cookies, &mut context);
    add_flash_messages_to_context(flash, &mut context);

    Template::render("user/login", &context.into_json())
}

#[post("/login", data = "<form>")]
fn login_post(
    mut cookies: Cookies,
    form: Form<Login>,
    database: State<StateHandler>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let user = database.get_user(&form.username);

    if !user.is_none() {
        let user = user.unwrap();
        if user.credentials.check_password(&form.password) {
            // User found - set cookie
            let mut cookie = Cookie::new("user", form.username.clone());
            cookie.set_path("/");
            cookies.add_private(cookie);

            let admin_level: u8 = user.admin_rights;
            if admin_level > 0 {
                let mut cookie = Cookie::new("admin", admin_level.to_string());
                cookie.set_path("/");
                cookies.add_private(cookie);
            }
            println!("Login ok");
            return Ok(Flash::success(
                Redirect::to("/"),
                format!("Welcome back, {}!", form.username),
            ));
        }
        // User not found or password wrong
    }
    println!("Login failed");
    return Err(Flash::error(
        Redirect::to("/user/login"),
        "Error: User not found or password did not match.",
    ));
}

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user"));
    Redirect::to("/")
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![login, login_post, logout]
}
