use rocket::serde::json::Json;

use crate::shared::{dtos::response::ResponseTaskDTO, structs::application::ListResponse};

use super::service;

#[get("/tasks?<per_page>&<page>")]
pub fn index(per_page: Option<usize>, page: Option<usize>) -> Json<ListResponse<ResponseTaskDTO>> {
    let response: ListResponse<ResponseTaskDTO> =
        service::execute(per_page.unwrap_or(10), page.unwrap_or(1));

    Json(response)
}
