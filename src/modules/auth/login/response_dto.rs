use std::io::Cursor;

use crate::shared::entities::users::User;
use chrono::NaiveDateTime;
use rocket::{
    http::{ContentType, Header, Status},
    response::Responder,
    Response,
};

#[derive(serde::Serialize)]
pub struct LoginUserDTO {
    id: uuid::Uuid,
    name: String,
    username: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}

impl LoginUserDTO {
    pub fn new(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }
    }
}

pub struct LoginResponder {
    user: LoginUserDTO,
    custom_header: Header<'static>,
}

impl LoginResponder {
    pub fn new(user: LoginUserDTO, token: String) -> Self {
        Self {
            user,
            custom_header: Header::new("Authorization", token),
        }
    }
}

impl<'r> Responder<'r, 'static> for LoginResponder {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let json = serde_json::to_string(&self.user).unwrap();

        Response::build()
            .header(ContentType::JSON)
            .header(self.custom_header)
            .sized_body(json.len(), Cursor::new(json))
            .status(Status::Ok)
            .ok()
    }
}
