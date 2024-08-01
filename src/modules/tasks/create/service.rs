use rocket::serde::json::Json;

use crate::shared::{
    entities::tasks::{NewTask, Task},
    repositories,
};

use super::request_dto::CreateTaskDTO;

pub fn execute(body: Json<CreateTaskDTO>) -> Task {
    let saved_task = repositories::tasks::create(NewTask {
        title: body.title.clone(),
        description: body.description.clone(),
        checked: body.checked.unwrap_or(false),
    });

    saved_task
}
