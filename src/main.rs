use rocket::{get, post, launch, routes, State};
use rocket::form::{Form, FromForm};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use rocket_dyn_templates::{Template, context};
use dotenv;

mod models;
use models::crud;


#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

// User registering
#[derive(FromForm)]
struct RegisterForm {
    csrf_token: String,
    username: String,
    password: String,
}

#[post("/register", data = "<form>")]
async fn register(db_conn: &State<Arc<DatabaseConnection>>, form: Form<RegisterForm>) -> &str {
    let _csrf_token: &str= &form.csrf_token; // TODO: Use csrf token
    let username: &str = &form.username;
    let password: &str = &form.password;

    match crud::insert_user(db_conn, username, password).await {
        Ok(_) => "User created.",
        Err(_) => "Error while creating user.",
    }
}

#[get("/register")]
async fn register_page() -> Template {
    Template::render("register", context! {csrf_token: "TODO"}) // TODO: Use csrf token
}

#[launch]
async fn rocket() -> _ {
    let database_url: String = dotenv::var("DATABASE_URL").unwrap();

    let db = Database::connect(database_url)
        .await
        .expect("Unable to connect to database.");

    let db_conn = Arc::new(db);

    rocket::build()
        .manage(db_conn)
        .mount("/", routes![index, register, register_page])
        .attach(Template::fairing())
}
