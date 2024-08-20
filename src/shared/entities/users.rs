use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable, Selectable};
use serde::Serialize;
use uuid;

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub username: Option<String>,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub username: Option<String>,
    pub password: String,
}
