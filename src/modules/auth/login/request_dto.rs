use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequestDTO {
    pub login: String,
    pub password: String,
}
