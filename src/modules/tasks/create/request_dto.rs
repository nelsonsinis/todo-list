use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CreateTaskDTO {
    pub title: String,
    pub description: Option<String>,
    pub checked: Option<bool>,
}
