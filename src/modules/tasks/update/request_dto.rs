use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UpdateTaskDTO {
    pub title: Option<String>,
    pub description: Option<String>,
    pub checked: Option<bool>,
}
