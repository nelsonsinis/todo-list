use super::response_dto::CreateUserResponseDTO;
use super::{request_dto::CreateUserDTO, service};
use crate::shared::dtos::response::CommonResponseDTO;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[post("/", data = "<body>")]
pub fn index(
    body: Json<CreateUserDTO>,
) -> Result<status::Custom<Json<CreateUserResponseDTO>>, status::Custom<Json<CommonResponseDTO>>> {
    match service::execute(body) {
        Ok(value) => Ok(status::Custom(
            Status::Created,
            Json(CreateUserResponseDTO::new(value)),
        )),
        Err(error) => Err(status::Custom(
            error.status.unwrap_or(Status::InternalServerError),
            Json(CommonResponseDTO::new(error.message)),
        )),
    }
}
