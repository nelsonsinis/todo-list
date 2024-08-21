#[macro_use]
extern crate rocket;
extern crate bcrypt;

mod modules;
pub mod schema;
mod shared;

use modules::{auth, tasks, users};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/auth", auth::routes())
        .mount("/users", users::routes())
        .mount("/tasks", tasks::routes())
}
