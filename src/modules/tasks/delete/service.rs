use rocket::http::Status;

use crate::shared::{repositories::tasks as repository, structs::application::ErrorResponse};

pub fn execute(id: &str) -> Result<(), ErrorResponse> {
    if let None = repository::find_by_id(id) {
        return Err(ErrorResponse {
            status: Some(Status::NotFound),
            message: "task-not-found",
        });
    }

    if !repository::delete_one(id) {
        return Err(ErrorResponse {
            status: Some(Status::InternalServerError),
            message: "cannot-possible-to-delete-task",
        });
    }

    Ok(())
}
