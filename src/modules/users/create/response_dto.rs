use chrono::NaiveDateTime;

use crate::shared::entities::users::User;

#[derive(serde::Serialize)]
pub struct CreateUserResponseDTO {
    id: uuid::Uuid,
    name: String,
    email: String,
    username: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}

impl CreateUserResponseDTO {
    pub fn new(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }
    }
}
