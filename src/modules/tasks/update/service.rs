use rocket::{http::Status, serde::json::Json};

use crate::shared::{
    entities::tasks::{Task, UpdateTask},
    repositories::tasks as repository,
    structs::application::ErrorResponse,
};

use super::request_dto::UpdateTaskDTO;

pub fn execute(id: &str, payload: Json<UpdateTaskDTO>) -> Result<Task, ErrorResponse> {
    let saved_task = repository::find_by_id(id);

    match saved_task {
        Some(value) => match repository::update_one(
            id,
            UpdateTask {
                title: if payload.title.is_none() {
                    value.title
                } else {
                    payload.title.clone().unwrap()
                },
                description: if payload.description.is_none() {
                    None
                } else {
                    payload.description.clone()
                },
                checked: if payload.checked.is_none() {
                    value.checked
                } else {
                    payload.checked.unwrap_or(false)
                },
            },
        ) {
            Ok(task) => Ok(task),
            Err(_) => Err(ErrorResponse {
                status: Some(Status::InternalServerError),
                message: "internal-error-to-update-a-task",
            }),
        },
        None => Err(ErrorResponse {
            status: Some(Status::NotFound),
            message: "task-not-found",
        }),
    }
}
