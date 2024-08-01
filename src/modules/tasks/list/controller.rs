use rocket::serde::json::Json;

use crate::shared::structs::application::ListResponse;

use super::{response_dto::TaskResponseDTO, service};

#[get("/tasks?<per_page>&<page>")]
pub fn index(
    mut per_page: Option<usize>,
    mut page: Option<usize>,
) -> Json<ListResponse<TaskResponseDTO>> {
    per_page = match per_page {
        Some(value) => Some(value),
        None => Some(10),
    };
    page = match page {
        Some(value) => Some(value),
        None => Some(1),
    };

    let response = service::list(per_page.unwrap(), page.unwrap());

    Json(response)
}
