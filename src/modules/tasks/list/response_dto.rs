use rocket::serde::Serialize;

use crate::shared::entities::tasks::Task;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TaskResponseDTO {
    id: uuid::Uuid,
    title: String,
    description: Option<String>,
    checked: bool,
}

impl TaskResponseDTO {
    pub fn new(task: Task) -> Self {
        Self {
            id: task.id,
            title: task.title,
            description: task.description,
            checked: task.checked,
        }
    }
}
