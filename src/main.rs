#[macro_use]
extern crate rocket;
extern crate bcrypt;

mod modules;
pub mod schema;
mod shared;

use modules::{tasks, users};

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            tasks::list::controller::index,
            tasks::create::controller::index,
            tasks::get::controller::index,
            tasks::delete::controller::index,
            tasks::update::controller::index,
            users::create::controller::index
        ],
    )
}
