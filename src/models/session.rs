use sea_orm::entity::prelude::*;
use sea_orm::prelude::Date;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key, unique, column_type = "Text")]
    pub session_token: String,
    
    #[sea_orm(auto_increment = false)]
    pub user_id: i32,
    
    #[sea_orm(column_type = "Date")]
    pub expire_date: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
