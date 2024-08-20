use crate::schema::users::{self, dsl::*};
use crate::shared::entities::users::NewUser;
use crate::shared::{entities::users::User, utils::database::establish_connection};
use diesel::prelude::*;

pub fn find_by_email_or_username(
    other_email: &String,
    other_username: &Option<String>,
) -> Option<User> {
    let connection = &mut establish_connection();
    let mut query = users.into_boxed();

    if let Some(value) = other_username {
        query = query.filter(email.eq(&other_email).or(username.eq(value)));
    }

    match query.select(User::as_select()).first(connection) {
        Err(_) => None,
        Ok(value) => Some(value),
    }
}

pub fn create(body: NewUser) -> User {
    let connection = &mut establish_connection();

    diesel::insert_into(users::table)
        .values(&body)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new user")
}
