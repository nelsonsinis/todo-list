use rocket::{http::Status, response::status, serde::json::Json};

use crate::shared::dtos::response::CommonResponseDTO;

use super::service;

#[delete("/<id>")]
pub fn index(id: &str) -> Result<status::NoContent, status::Custom<Json<CommonResponseDTO>>> {
    match service::execute(id) {
        Ok(_) => Ok(status::NoContent),
        Err(resp) => Err(status::Custom(
            resp.status.unwrap_or(Status::InternalServerError),
            Json(CommonResponseDTO::new(resp.message)),
        )),
    }
}
