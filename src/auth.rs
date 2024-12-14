use rocket::form::{Form, FromForm};
use rocket::response::Redirect;
use rocket::{get, post, State};
use rocket_dyn_templates::{context, Template};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use rocket::http::{Cookie, CookieJar};

use crate::models::crud;

// User registering
#[derive(FromForm)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
}

#[post("/register", data = "<form>")]
pub async fn register(db_conn: &State<Arc<DatabaseConnection>>, form: Form<RegisterForm>, cookies: &CookieJar<'_>) -> Redirect {
    let email = &form.email;
    let password = &form.password;

    match crud::insert_user(db_conn, email, password).await {
        Ok(_) => {
            if let Some(session) = crud::get_or_create_session(db_conn, crud::get_user_by_email(db_conn, &form.email).await.unwrap()).await {
                cookies.add(Cookie::new("session_token", session.session_token));
                Redirect::to("/")
            } else {
                Redirect::to("/register")
            }
        },
        Err(_) => Redirect::to("/register"),
    }
}

#[get("/register")]
pub async fn register_page() -> Template {
    Template::render("register", context! {})
}

// User login
#[derive(FromForm)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[post("/login", data = "<form>")]
pub async fn login(db_conn: &State<Arc<DatabaseConnection>>, form: Form<LoginForm>, cookies: &CookieJar<'_>) -> Redirect {
    if crud::check_credential(db_conn, &form.email, &form.password).await {
        if let Some(session) = crud::get_or_create_session(db_conn, crud::get_user_by_email(db_conn, &form.email).await.unwrap()).await {
            cookies.add(Cookie::new("session_token", session.session_token));
            Redirect::to("/")
        } else {
            Redirect::to("/login")
        }
    } else {
        Redirect::to("/login")
    }
}

#[get("/login")]
pub async fn login_page() -> Template {
    Template::render("login", context! {})
}

#[macro_export]
macro_rules! requires_login {
    ($db_conn:expr, $cookies:expr) => {
        use crate::models::crud;
        use rocket::response::Redirect;

        if let Some(cookie) = $cookies.get("session_token") {
            let token = cookie.value();
    
            if !crud::is_valid_session_token($db_conn, token).await {
                return Err(Redirect::to("/login"));
            }
        } else {
            return Err(Redirect::to("/login"));
        }
    };
}