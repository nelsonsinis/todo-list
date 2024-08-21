use rocket::{http::Status, response::status, serde::json::Json};

use crate::shared::dtos::response::{CommonResponseDTO, ResponseTaskDTO};

use super::service;

#[get("/<id>")]
pub fn index(id: &str) -> Result<Json<ResponseTaskDTO>, status::Custom<Json<CommonResponseDTO>>> {
    match service::execute(id) {
        Err(resp) => Err(status::Custom(
            resp.status.unwrap_or(Status::InternalServerError),
            Json(CommonResponseDTO::new(resp.message)),
        )),
        Ok(task) => Ok(Json(ResponseTaskDTO::new(task))),
    }
}
