use rocket::http::{Cookie, Cookies};
use rocket::request::{FlashMessage, Form};
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;
use tera::Context;
use uuid::Uuid;

use crate::database::database_model::StateHandler;
use crate::{add_flash_messages_to_context, add_user_cookie_to_context, database};

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
    //add_user_cookie_to_context(cookies, &mut context);
    Template::render("posts/create", &context.into_json())
}

#[get("/")]
pub fn index(
    flash: Option<FlashMessage>,
    mut cookies: Cookies,
    database: State<StateHandler>,
) -> Template {
    let mut context = Context::new();

    // call check_user_cookie
    add_user_cookie_to_context(cookies, &mut context);
    add_flash_messages_to_context(flash, &mut context);

    let mut posts: Vec<Post> = vec![];
    for post in database.posts.lock().unwrap().iter().rev() {
        posts.push(post.clone());
    }

    context.insert("posts", &posts);

    // context.insert("notifications", &vec!["Test Notification"]);
    // context.insert("errors", &vec!["Cant login", "Error"]);
    Template::render("posts/index", &context.into_json())
}

#[post("/create", data = "<form>")]
fn create_post(
    mut cookies: Cookies,
    form: Form<PostForm>,
    database: State<StateHandler>,
) -> Redirect {
    let user = cookies.get_private("user");
    if user.is_none() {
        println!("create_post: User not logged in");
        return Redirect::to("/user/login");
    }
    let post: Post = Post::new(
        Uuid::new_v4().to_string(),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        user.unwrap().value().to_string(),
        form.title.clone(),
        form.content.clone(),
    );
    let data = database.inner();
    data.posts.lock().unwrap().push(post);

    database.save_post_database();

    // println!("{}", &form.title);
    // println!("{}", &form.content);
    return Redirect::to("/");
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![index, create, create_post]
}
