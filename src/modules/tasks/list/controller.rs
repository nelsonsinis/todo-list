use rocket::serde::json::Json;

use crate::shared::structs::application::ListResponse;

use super::{response_dto::TaskResponseDTO, service};

#[get("/tasks?<per_page>&<page>")]
pub fn index(per_page: Option<usize>, page: Option<usize>) -> Json<ListResponse<TaskResponseDTO>> {
    let response = service::list(per_page.unwrap_or(10), page.unwrap_or(1));

    Json(response)
}
