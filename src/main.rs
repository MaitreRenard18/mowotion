use dotenv::dotenv;
use models::crud;
use rocket::{fs::FileServer, get, launch, routes, State};
use rocket_dyn_templates::{context, Template};
use rocket::http::{Cookie, CookieJar};
use std::sync::Arc;
use sea_orm::DatabaseConnection;
use rocket::response::Redirect;

mod auth;
mod config;
mod models;

use auth::{login, login_page, logout, register, register_page};
use config::init_database;

#[get("/")]
async fn index(db_conn: &State<Arc<DatabaseConnection>>, cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    requires_login!(db_conn, cookies); 

    if let Some(cookie) = cookies.get("session_token") {
        if let Some(user) = crud::get_user_by_session(db_conn, cookie.value()).await {
            return Ok(Template::render("index", context! {email: user.email}));
        }
    }

    return Err(Redirect::to("/login"));
}

#[get("/logged")]
async fn logged(db_conn: &State<Arc<DatabaseConnection>>, cookies: &CookieJar<'_>) -> Result<&'static str, Redirect> {
    requires_login!(db_conn, cookies);
    Ok("You are logged in.")
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok(); // Load .env variables

    let db_conn = init_database().await;

    rocket::build()
        .manage(db_conn)
        .mount(
            "/",
            routes![index, register, register_page, login, login_page, logged, logout],
        )
        .mount("/static", FileServer::from("./static"))
        .attach(Template::fairing())
}
