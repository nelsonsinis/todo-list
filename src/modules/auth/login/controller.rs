use rocket::{http::Status, response::status, serde::json::Json};

use crate::shared::dtos::response::CommonResponseDTO;

use super::{
    request_dto::LoginRequestDTO,
    response_dto::{LoginResponder, LoginUserDTO},
    service,
};

#[post("/auth/login", data = "<body>")]
pub fn index(
    body: Json<LoginRequestDTO>,
) -> Result<
    LoginResponder,
    rocket::response::status::Custom<rocket::serde::json::Json<CommonResponseDTO>>,
> {
    match service::execute(body) {
        Err(error) => Err(status::Custom(
            error.status.unwrap_or(Status::InternalServerError),
            Json(CommonResponseDTO::new(error.message)),
        )),
        Ok(response) => Ok(LoginResponder::new(
            LoginUserDTO::new(response.user),
            response.token,
        )),
    }
}
