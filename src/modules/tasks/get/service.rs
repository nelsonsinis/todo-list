use rocket::http::Status;

use crate::shared::{
    entities::tasks::Task, repositories::tasks as repository, structs::application::ErrorResponse,
};

pub fn execute(id: &str) -> Result<Task, ErrorResponse> {
    let task = repository::find_by_id(id);

    match task {
        Some(value) => Ok(value),
        None => Err(ErrorResponse {
            status: Some(Status::NotFound),
            message: "task-not-found",
        }),
    }
}
