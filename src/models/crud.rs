use sea_orm::{ActiveModelTrait, DatabaseConnection, ActiveValue};
use super::user;

pub async fn insert_user(db_conn: &DatabaseConnection, username: &str, password: &str) -> Result<user::Model, sea_orm::DbErr> {
    let new_user = user::ActiveModel {
        username: ActiveValue::Set(username.to_string()),
        password: ActiveValue::Set(password.to_string()),
        ..Default::default()
    };

    new_user.insert(db_conn).await
}
