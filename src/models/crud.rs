use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DeleteResult};
use sea_orm::{EntityTrait, QueryFilter};

use super::user::Entity as UserEntity;
use super::user::Model as UserModel;
use super::user::{ActiveModel as User, Column as UserColumn};
use super::session::{ActiveModel as Session, Column as SessionColumn};
use super::session::Entity as SessionEntity;
use super::session::Model as SessionModel;
use super::utils::{self, generate_csrf_token, generate_session_token};

pub async fn insert_user(db_conn: &DatabaseConnection, email: &str, password: &str) -> Result<UserModel, sea_orm::DbErr> {
    let new_user = User {
        email: ActiveValue::Set(email.to_string()),
        password: ActiveValue::Set(utils::hash_password(password).await),
        ..Default::default()
    };

    new_user.insert(db_conn).await
}

pub async fn check_credential(db_conn: &DatabaseConnection, email: &str, password: &str) -> bool {
    let user = UserEntity::find()
        .filter(UserColumn::Email.eq(email))
        .one(db_conn)
        .await
        .unwrap();

    if let Some(found_user) = user {
        found_user.password == utils::hash_password(password).await
    } else {
        false
    }
}

pub async fn get_user_by_email(db_conn: &DatabaseConnection, email: &str) -> Option<UserModel> {
    UserEntity::find()
        .filter(UserColumn::Email.eq(email))
        .one(db_conn)
        .await
        .unwrap()
}

pub async fn get_or_create_session(db_conn: &DatabaseConnection, user: UserModel) -> Option<SessionModel> {
    let session = SessionEntity::find()
        .filter(SessionColumn::Id.eq(user.id))
        .one(db_conn)
        .await
        .unwrap();
    
    if let Some(s) = session {
        Some(s)
    } else {
        let new_session = Session {
            id: ActiveValue::Set(user.id),
            session_token: ActiveValue::Set(generate_session_token().await),
            csrf_token: ActiveValue::Set(generate_csrf_token().await),
            ..Default::default()
        };
        
        match new_session.insert(db_conn).await {
            Ok(s) => Some(s),
            Err(_) => None
        }
    }
}

pub async fn is_valid_session_token(db_conn: &DatabaseConnection, session_token: &str) -> bool {
    let session = SessionEntity::find()
        .filter(SessionColumn::SessionToken.eq(session_token))
        .one(db_conn)
        .await
        .unwrap();

    if let Some(_) = session {
        true
    } else {
        false
    }
}

pub async fn delete_session(db_conn: &DatabaseConnection, session_token: &str) -> Result<DeleteResult, DbErr> {
    println!("{}", session_token);

    SessionEntity::delete_many()
        .filter(SessionColumn::SessionToken.eq(session_token))
        .exec(db_conn)
        .await
}


pub async fn get_user_by_session(db_conn: &DatabaseConnection, session_token: &str) -> Option<UserModel> {
    let session = SessionEntity::find()
        .filter(SessionColumn::SessionToken.eq(session_token))
        .one(db_conn)
        .await
        .unwrap();

    if let Some(s) = session {
        UserEntity::find()
            .filter(UserColumn::Id.eq(s.id))
            .one(db_conn)
            .await
            .unwrap()
    } else {
        None
    }
}
