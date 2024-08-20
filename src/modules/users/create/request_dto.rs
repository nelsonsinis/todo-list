use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CreateUserDTO {
    pub name: String,
    pub email: String,
    pub username: Option<String>,
    pub password: String
}