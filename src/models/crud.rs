use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DeleteResult};
use sea_orm::{EntityTrait, QueryFilter};

use super::user::Entity as UserEntity;
use super::user::Model as UserModel;
use super::user::{ActiveModel as User, Column as UserColumn};
use super::session::{ActiveModel as Session, Column as SessionColumn};
use super::session::Entity as SessionEntity;
use super::session::Model as SessionModel;
use super::utils::{hash_password, generate_session_token, get_current_date, get_expire_date};

pub async fn insert_user(db_conn: &DatabaseConnection, email: &str, password: &str) -> Result<UserModel, sea_orm::DbErr> {
    let new_user = User {
        email: ActiveValue::Set(email.to_string()),
        password: ActiveValue::Set(hash_password(password).await),
        ..Default::default()
    };

    new_user.insert(db_conn).await
}

pub async fn check_credential(db_conn: &DatabaseConnection, email: &str, password: &str) -> bool {
    let user = get_user_by_email(db_conn, email).await;

    if let Some(found_user) = user {
        found_user.password == hash_password(password).await
    } else {
        false
    }
}

pub async fn get_user_by_id(db_conn: &DatabaseConnection, id: i32) -> Option<UserModel> {
    match UserEntity::find().filter(UserColumn::Id.eq(id)).one(db_conn).await {
        Ok(user) => user,
        Err(_) => None
    }
}

pub async fn get_user_by_email(db_conn: &DatabaseConnection, email: &str) -> Option<UserModel> {
    match  UserEntity::find().filter(UserColumn::Email.eq(email)).one(db_conn).await {
        Ok(user) => user,
        Err(_) => None
    }
}

pub async fn get_or_create_session(db_conn: &DatabaseConnection, user: UserModel) -> Option<SessionModel> {
    let session = get_session_by_user(db_conn, &user).await;
    
    if let Some(s) = session {
        Some(s)
    } else {
        let new_session = Session {
            user_id: ActiveValue::Set(user.id),
            session_token: ActiveValue::Set(generate_session_token().await),
            expire_date: ActiveValue::Set(get_expire_date().await),
            ..Default::default()
        };
        
        match new_session.insert(db_conn).await {
            Ok(s) => Some(s),
            Err(_) => None
        }
    }
}

pub async fn get_session_by_user(db_conn: &DatabaseConnection, user: &UserModel) -> Option<SessionModel> {
    match SessionEntity::find().filter(SessionColumn::UserId.eq(user.id)).one(db_conn).await {
        Ok(session) => session,
        Err(_) => None
    }
}

pub async fn get_session_by_token(db_conn: &DatabaseConnection, session_token: &str) -> Option<SessionModel> {
    match SessionEntity::find().filter(SessionColumn::SessionToken.eq(session_token)).one(db_conn).await {
        Ok(session) => session,
        Err(_) => None
    }
}

pub async fn is_valid_session_token(db_conn: &DatabaseConnection, session_token: &str) -> bool {
    let session_option = get_session_by_token(db_conn, session_token).await;
    
    if let Some(session) = session_option {
        session.expire_date >= get_current_date().await
    } else {
        false
    }
}

pub async fn delete_session(db_conn: &DatabaseConnection, session_token: &str) -> Result<DeleteResult, DbErr> {
    SessionEntity::delete_many()
        .filter(SessionColumn::SessionToken.eq(session_token))
        .exec(db_conn)
        .await
}

pub async fn get_user_by_session(db_conn: &DatabaseConnection, session_token: &str) -> Option<UserModel> {
    let session_option = get_session_by_token(db_conn, session_token).await;
    
    if let Some(session) = session_option {
        get_user_by_id(db_conn, session.user_id).await
    } else {
        None
    }
}
