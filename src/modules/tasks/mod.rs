use rocket::Route;

mod create;
mod delete;
mod get;
mod list;
mod update;

pub fn routes() -> Vec<Route> {
    routes![
        create::controller::index,
        delete::controller::index,
        get::controller::index,
        list::controller::index,
        update::controller::index,
    ]
}
