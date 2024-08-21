use crate::shared::{
    entities::users::User, repositories::users as repository, structs::application::ErrorResponse,
    utils::jwt::JWT,
};
use bcrypt::verify;
use rocket::{http::Status, serde::json::Json};

use super::request_dto::LoginRequestDTO;

pub struct ServiceResponse {
    pub token: String,
    pub user: User,
}

pub fn execute(body: Json<LoginRequestDTO>) -> Result<ServiceResponse, ErrorResponse<'static>> {
    let error_response = ErrorResponse {
        status: Some(Status::Unauthorized),
        message: "unauthorized-access",
    };

    match repository::find_by_email_or_username(&body.login, &Some(body.login.clone())) {
        None => Err(error_response),
        Some(user) => match verify(&body.password, &user.password) {
            Ok(result) => {
                let mut jwt = JWT::new(user.id);

                if result {
                    Ok(ServiceResponse {
                        token: jwt.sign(None),
                        user,
                    })
                } else {
                    Err(error_response)
                }
            }
            Err(_) => Err(error_response),
        },
    }
}
