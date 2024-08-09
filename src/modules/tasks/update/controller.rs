use rocket::{http::Status, response::status, serde::json::Json};

use crate::shared::dtos::response::{CommonResponseDTO, ResponseTaskDTO};

use super::{request_dto::UpdateTaskDTO, service};

#[patch("/tasks/<id>", data = "<body>")]
pub fn index(
    id: &str,
    body: Json<UpdateTaskDTO>,
) -> Result<Json<ResponseTaskDTO>, status::Custom<Json<CommonResponseDTO>>> {
    match service::execute(id, body) {
        Ok(task) => Ok(Json(ResponseTaskDTO::new(task))),
        Err(error) => Err(status::Custom(
            error.status.unwrap_or(Status::InternalServerError),
            Json(CommonResponseDTO::new(error.message)),
        )),
    }
}
