use rocket::{get, launch, routes};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;

mod models;

const DATABASE_URL: &str = "sqlite:./nowotion.db?mode=rwc";

type DbConn = Arc<DatabaseConnection>;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    // Connexion à la base de données
    let db = Database::connect(DATABASE_URL)
        .await
        .expect("Impossible de se connecter à la base de données");

    let db_conn = Arc::new(db);

    rocket::build()
        .manage(db_conn)
        .mount("/", routes![index])
}
