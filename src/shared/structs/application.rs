use rocket::{http::Status, serde::Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ListResponse<T> {
    pub per_page: usize,
    pub total_page: usize,
    pub items: Vec<T>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse<'a> {
    pub status: Option<Status>,
    pub message: &'a str,
}
