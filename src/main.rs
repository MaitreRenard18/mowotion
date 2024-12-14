use dotenv::dotenv;
use rocket::{get, launch, routes, State};
use rocket_dyn_templates::Template;
use rocket::http::{Cookie, CookieJar};
use std::sync::Arc;
use sea_orm::DatabaseConnection;

mod auth;
mod config;
mod models;

use auth::{login, login_page, register, register_page};
use config::init_database;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[get("/logged")]
async fn logged(db_conn: &State<Arc<DatabaseConnection>>, cookies: &CookieJar<'_>) -> &'static str {
    if let Some(cookie) = cookies.get("session_token") {
        let token = cookie.value();

        if models::crud::is_valid_session_token(db_conn, token).await {
            "Yes"
        } else {
            "No."
        }
    } else {
        "No."
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok(); // Load .env variables

    let db_conn = init_database().await;

    rocket::build()
        .manage(db_conn)
        .mount(
            "/",
            routes![index, register, register_page, login, login_page, logged],
        )
        .attach(Template::fairing())
}
