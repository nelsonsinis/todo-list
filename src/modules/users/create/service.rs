use crate::modules::users::create::request_dto::CreateUserDTO;
use crate::shared::entities::users::NewUser;
use crate::shared::structs::application::ErrorResponse;
use crate::shared::{entities::users::User, repositories::users as repository};
use bcrypt::{hash, DEFAULT_COST};
use rocket::http::Status;
use rocket::serde::json::Json;

pub fn execute(body: Json<CreateUserDTO>) -> Result<User, ErrorResponse<'static>> {
    match repository::find_by_email_or_username(&body.email, &body.username) {
        Some(_) => Err(ErrorResponse {
            status: Some(Status::Conflict),
            message: "username-or-email-already-exists",
        }),
        None => Ok(repository::create(NewUser {
            name: body.name.clone(),
            email: body.email.clone(),
            username: body.username.clone(),
            password: hash(&body.password, DEFAULT_COST).unwrap(),
        })),
    }
}
