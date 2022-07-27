use rocket::http::{Cookie, Cookies, RawStr, Status};
use rocket::outcome::Outcome::*;
use rocket::request::{self, FlashMessage, Form, FromRequest, Request, State};
use rocket::response::{Flash, Redirect};
use rocket::Outcome;
use rocket_contrib::templates::Template;
use tera::Context;

use crate::database::database_model::StateHandler;
use crate::user::user_model::User;
use crate::{
    add_flash_messages_to_context, add_user_cookie_to_context, database, FlashNotification,
};

use self::user_model::{AuthenticatedUser, UserCredentials, UserProfile};

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

#[derive(FromForm)]
pub struct SettingsProfileForm {
    pub profile_name: String,
    pub profile_bio: String,
    pub profile_twitter: String,
    pub profile_github: String,
}

#[derive(FromForm)]
pub struct SettingsPasswordForm {
    pub password_old: String,
    pub password_new: String,
    pub password_confirm: String,
}

#[derive(FromForm)]
pub struct SettingsAddUserForm {
    pub user_name: String,
    pub user_password: String,
    pub user_admin: bool,
}

#[derive(FromForm)]
pub struct SettingsDeleteUserForm {
    pub user_name: String,
}

#[derive(FromForm)]
pub struct SettingsAdminUserForm {
    pub user_name: String,
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
    cookies.remove_private(Cookie::named("admin"));
    Redirect::to("/")
}

#[get("/profile/<username>")]
fn profile(
    username: String,
    cookies: Cookies,
    database: State<StateHandler>,
) -> Result<Template, Status> {
    let mut context = Context::new();
    add_user_cookie_to_context(cookies, &mut context);

    // Fetch profile infos
    let user = database.get_user(&username);
    if !user.is_none() {
        let user = user.unwrap();
        context.insert("username", &user.name);

        context.insert("userhandle", &username);

        let user_profile = UserProfile {
            bio: user.profile.bio,
            twitter: user.profile.twitter,
            github: user.profile.github,
        };
        context.insert("profile", &user_profile);

        // Get 5 latest posts from user
        let posts = database.get_posts_by_user(&username);
        context.insert("posts", &posts);

        return Ok(Template::render("user/profile", &context.into_json()));
    }
    return Err(Status::NotFound);
}

#[get("/profile")]
fn your_profile(
    user: AuthenticatedUser,
    cookies: Cookies,
    database: State<StateHandler>,
) -> Template {
    profile(user.username, cookies, database).unwrap()
}

#[get("/settings")]
fn settings(
    user: AuthenticatedUser,
    flash: Option<FlashMessage>,
    cookies: Cookies,
    database: State<StateHandler>,
) -> Template {
    let mut context = Context::new();
    add_user_cookie_to_context(cookies, &mut context);
    add_flash_messages_to_context(flash, &mut context);

    // Fetch profile infos
    let user = database.get_user(&user.username);
    if !user.is_none() {
        let user_data = user.unwrap();
        let user = User {
            name: user_data.name,
            credentials: UserCredentials {
                username: user_data.credentials.username,
                password_hash: "".to_string(),
            },
            profile: user_data.profile,
            admin_rights: user_data.admin_rights,
        };
        context.insert("user", &user);

        // Hand over users if the user is an admin
        if user_data.admin_rights == 1 {
            let mut users: Vec<User> = vec![];
            for user in database.users.lock().unwrap().iter() {
                let user_data = User {
                    name: user.name.clone(),
                    credentials: UserCredentials {
                        username: user.credentials.username.clone(),
                        password_hash: "".to_string(),
                    },
                    profile: UserProfile {
                        bio: "".to_string(),
                        twitter: "".to_string(),
                        github: "".to_string(),
                    },
                    admin_rights: user.admin_rights,
                };
                users.push(user_data);
            }
            context.insert("users", &users);
        }
    }

    Template::render("user/settings", &context.into_json())
}

#[post("/update_profile", data = "<form>")]
fn update_profile(
    user: AuthenticatedUser,
    form: Form<SettingsProfileForm>,
    database: State<StateHandler>,
) -> Result<Redirect, Status> {
    let user_id = database.get_user_id_by_username(user.username.clone());
    let user_data = database.get_user_by_id(user_id.unwrap());

    if user_data.is_some() {
        let user_data = user_data.unwrap();

        if user.username == user_data.credentials.username {
            let user = User {
                name: form.profile_name.clone(),
                credentials: UserCredentials {
                    username: user_data.credentials.username.clone(),
                    password_hash: user_data.credentials.password_hash.clone(),
                },
                profile: UserProfile {
                    bio: form.profile_bio.clone(),
                    twitter: form.profile_twitter.clone(),
                    github: form.profile_github.clone(),
                },
                admin_rights: user_data.admin_rights,
            };

            let data = database.inner();
            data.users.lock().unwrap()[user_id.unwrap() as usize] = user;
            database.save_user_database();

            return Ok(Redirect::to("/user/settings"));
        } else {
            return Err(Status::Unauthorized);
        }
    }

    Ok(Redirect::to("/user/settings"))
}

#[post("/update_password", data = "<form>")]
fn update_password(
    user: AuthenticatedUser,
    form: Form<SettingsPasswordForm>,
    database: State<StateHandler>,
) -> Flash<Redirect> {
    let user_id = database.get_user_id_by_username(user.username.clone());
    let user_data = database.get_user_by_id(user_id.unwrap());

    if user_data.is_some() {
        let user_data = user_data.unwrap();

        // Check if the logged in user is the same as the user in the database
        if user.username == user_data.credentials.username {
            // Check if the current password is correct
            if user_data.credentials.check_password(&form.password_old) {
                if form.password_confirm == form.password_new {
                    let password_hash = UserCredentials::calc_password_hash(&form.password_new);
                    let data = database.inner();
                    data.users.lock().unwrap()[user_id.unwrap() as usize]
                        .credentials
                        .password_hash = password_hash;
                    database.save_user_database();
                    return Flash::success(
                        Redirect::to("/"),
                        "Password has been changed successfully.",
                    );
                } else {
                    return Flash::error(
                        Redirect::to("/user/settings"),
                        "New password does not match the confirm password.",
                    );
                }
            } else {
                return Flash::error(
                    Redirect::to("/user/settings"),
                    "Current password does not match.",
                );
            }
        }
    }

    return Flash::error(Redirect::to("/user/settings"), "General Error");
}

#[post("/add_user", data = "<form>")]
fn add_user(
    user: AuthenticatedUser,
    form: Form<SettingsAddUserForm>,
    database: State<StateHandler>,
) -> Flash<Redirect> {
    let user_id = database.get_user_id_by_username(user.username.clone());
    let user_data = database.get_user_by_id(user_id.unwrap());

    // Check if current user has admin rights
    if user_data.is_none() {
        return Flash::error(Redirect::to("/user/settings"), "General Error");
    }
    let user_data = user_data.unwrap();
    if user_data.admin_rights != 1 {
        return Flash::error(Redirect::to("/user/settings"), "General Error");
    }

    // Check if user name is unique
    if database
        .get_user_id_by_username(form.user_name.clone())
        .is_some()
    {
        return Flash::error(
            Redirect::to("/user/settings"),
            "User with that name already exists.",
        );
    }

    // Add user to database
    let admin_right = if form.user_admin { 1 } else { 0 };
    let password_hash = UserCredentials::calc_password_hash(&form.user_password);
    let user = User {
        name: form.user_name.clone(),
        credentials: UserCredentials {
            username: form.user_name.clone(),
            password_hash: password_hash,
        },
        profile: UserProfile {
            bio: "".to_string(),
            twitter: "".to_string(),
            github: "".to_string(),
        },
        admin_rights: admin_right,
    };

    let data = database.inner();
    data.users.lock().unwrap().push(user);
    database.save_user_database();

    return Flash::success(Redirect::to("/user/settings"), "User has been created.");
}

#[post("/delete_user", data = "<form>")]
fn delete_user(
    user: AuthenticatedUser,
    form: Form<SettingsDeleteUserForm>,
    database: State<StateHandler>,
) -> Flash<Redirect> {
    let user_id = database.get_user_id_by_username(user.username.clone());
    let user_data = database.get_user_by_id(user_id.unwrap());

    // Check if current user has admin rights
    if user_data.is_none() {
        return Flash::error(Redirect::to("/user/settings"), "General Error");
    }
    let user_data = user_data.unwrap();
    if user_data.admin_rights != 1 {
        return Flash::error(Redirect::to("/user/settings"), "General Error");
    }

    // Remove user from database
    let delete_user_id = database.get_user_id_by_username(form.user_name.clone());
    if delete_user_id.is_some() {
        let data = database.inner();
        data.users
            .lock()
            .unwrap()
            .remove(delete_user_id.unwrap() as usize);
        database.save_user_database();
    }

    return Flash::success(Redirect::to("/user/settings"), "User has been deleted.");
}

#[post("/update_admin", data = "<form>")]
fn update_admin(
    user: AuthenticatedUser,
    form: Form<SettingsAdminUserForm>,
    database: State<StateHandler>,
) -> Flash<Redirect> {
    let user_id = database.get_user_id_by_username(user.username.clone());
    let user_data = database.get_user_by_id(user_id.unwrap());

    // Check if current user has admin rights
    if user_data.is_none() {
        return Flash::error(Redirect::to("/user/settings"), "General Error");
    }
    let user_data = user_data.unwrap();
    if user_data.admin_rights != 1 {
        return Flash::error(Redirect::to("/user/settings"), "General Error");
    }

    // Update admin rights
    let update_user_id = database.get_user_id_by_username(form.user_name.clone());
    let update_user_data = database.get_user_by_id(update_user_id.unwrap());

    if update_user_data.is_some() {
        let update_user_data = update_user_data.unwrap();
        let admin_right = if update_user_data.admin_rights == 1 {
            0
        } else {
            1
        };
        let data = database.inner();
        data.users
            .lock()
            .unwrap()
            .get_mut(update_user_id.unwrap() as usize)
            .unwrap()
            .admin_rights = admin_right;
        database.save_user_database();
    }

    return Flash::success(
        Redirect::to("/user/settings"),
        "Admin rights have been changed successfully.",
    );
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        login,
        login_post,
        logout,
        your_profile,
        profile,
        settings,
        update_profile,
        update_password,
        add_user,
        delete_user,
        update_admin
    ]
}
