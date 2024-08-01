use rocket::{http::Status, response::status, serde::json::Json};

use crate::shared::entities::tasks::Task;

use super::{request_dto::CreateTaskDTO, service};

#[post("/tasks", data = "<body>")]
pub fn index(body: Json<CreateTaskDTO>) -> status::Custom<Json<Task>> {
    let task = service::execute(body);

    status::Custom(Status::Created, Json(task))
}
