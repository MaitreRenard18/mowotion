use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;

pub async fn init_database() -> Arc<DatabaseConnection> {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to the database");
    Arc::new(db)
}
