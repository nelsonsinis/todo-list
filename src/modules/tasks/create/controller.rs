use rocket::{http::Status, response::status, serde::json::Json};

use crate::shared::dtos::response::ResponseTaskDTO;

use super::{request_dto::CreateTaskDTO, service};

#[post("/tasks", data = "<body>")]
pub fn index(body: Json<CreateTaskDTO>) -> status::Custom<Json<ResponseTaskDTO>> {
    let task = service::execute(body);

    status::Custom(Status::Created, Json(ResponseTaskDTO::new(task)))
}
