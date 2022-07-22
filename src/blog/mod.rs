use rocket::http::{Cookie, Cookies};
use rocket::request::{FlashMessage, Form};
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;
use tera::Context;
use uuid::Uuid;

use crate::database::database_model::StateHandler;
use crate::user::user_model::AuthenticatedUser;
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

#[get("/<page>")]
pub fn index(
    page: u32,
    flash: Option<FlashMessage>,
    mut cookies: Cookies,
    database: State<StateHandler>,
) -> Template {
    let mut context = Context::new();

    // call check_user_cookie
    add_user_cookie_to_context(cookies, &mut context);
    add_flash_messages_to_context(flash, &mut context);

    let mut posts: Vec<Post> = vec![];

    let entries_per_page = 5;

    for i in (page * entries_per_page)..(page * entries_per_page + entries_per_page) {
        if i < database.posts.lock().unwrap().len() as u32 {
            posts.push(database.posts.lock().unwrap()[i as usize].clone());
        }
    }
    let total_pages: u32 = (database.posts.lock().unwrap().len() + 1) as u32 / 2;
    context.insert("posts", &posts);
    context.insert("current_page", &page);
    context.insert("total_pages", &total_pages);
    Template::render("posts/index", &context.into_json())
}

#[get("/create")]
fn create(_user: AuthenticatedUser, cookies: Cookies) -> Template {
    let mut context = Context::new();
    add_user_cookie_to_context(cookies, &mut context);
    Template::render("posts/create", &context.into_json())
}

#[post("/create", data = "<form>")]
fn create_post(
    user: AuthenticatedUser,
    form: Form<PostForm>,
    database: State<StateHandler>,
) -> Redirect {
    let post: Post = Post::new(
        Uuid::new_v4().to_string(),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        user.username.clone(),
        form.title.clone(),
        form.content.clone(),
    );
    let data = database.inner();
    data.posts.lock().unwrap().reverse();
    data.posts.lock().unwrap().push(post);
    data.posts.lock().unwrap().reverse();

    database.save_post_database();
    return Redirect::to("/");
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![index, create, create_post]
}
