#[macro_use]
extern crate rocket;

mod modules;
pub mod schema;
mod shared;

use modules::tasks;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            tasks::list::controller::index,
            tasks::create::controller::index,
            tasks::get::controller::index,
            tasks::delete::controller::index
        ],
    )
}
