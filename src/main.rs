use rocket::{get, launch, routes};
use sea_orm::Database;
use std::sync::Arc;

mod models;

const DATABASE_URL: &str = "sqlite:./mowotion.db?mode=rwc";

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let db = Database::connect(DATABASE_URL).await;

    rocket::build()
        .manage(Arc::new(db))
        .mount("/", routes![index])
}
