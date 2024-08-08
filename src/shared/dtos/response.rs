use chrono::NaiveDateTime;
use rocket::serde;

use crate::shared::entities::tasks::Task;

#[derive(serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CommonResponseDTO {
    message: String,
}

impl CommonResponseDTO {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseTaskDTO {
    id: uuid::Uuid,
    title: String,
    description: Option<String>,
    checked: bool,
    deleted_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl ResponseTaskDTO {
    pub fn new(task: Task) -> Self {
        Self {
            id: task.id,
            title: task.title,
            description: task.description,
            checked: task.checked,
            deleted_at: task.deleted_at,
            updated_at: task.updated_at,
            created_at: task.created_at,
        }
    }
}
