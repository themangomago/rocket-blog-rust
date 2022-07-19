use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;
use tera::Context;

use crate::database::database_model::StateHandler;
use crate::{database, get_user_cookie};

use self::post_model::Post;

#[path = "post_model.rs"]
pub mod post_model;

////////////////////////////////////////////////////////////////////////////////
/// Routes
////////////////////////////////////////////////////////////////////////////////

#[derive(FromForm)]
pub struct PostForm {
    pub title: String,
    pub content: String,
}

#[get("/create")]
fn create(mut cookies: Cookies) -> Template {
    if !cookies.get_private("user").is_some() {
        println!("create: User not logged in");
    }

    let mut context = Context::new();
    get_user_cookie(cookies, &mut context);
    Template::render("posts/create", &context.into_json())
}

#[post("/create", data = "<form>")]
fn create_post(
    mut cookies: Cookies,
    form: Form<PostForm>,
    database: State<StateHandler>,
) -> Redirect {
    if !cookies.get_private("user").is_some() {
        // TODO: with flash
        return Redirect::to("/user/login");
    }

    let post: Post = Post::new(
        "uuid".to_string(),
        "authorname".to_string(),
        form.title.clone(),
        form.content.clone(),
    );
    let data = database.inner();
    data.posts.lock().unwrap().push(post);
    println!("Number of posts: {}", data.posts.lock().unwrap().len());
    database.save_post_database();

    // println!("{}", &form.title);
    // println!("{}", &form.content);
    return Redirect::to("/");
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![create, create_post]
}
