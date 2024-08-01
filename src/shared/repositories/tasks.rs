use diesel::{dsl::count, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    schema::tasks::dsl::*,
    shared::{entities::tasks::Task, utils::database::establish_connection},
};

pub fn find(per_page: usize, page: usize) -> (i64, Vec<Task>) {
    let connection = &mut establish_connection();

    let results: Vec<Task> = tasks
        .select(Task::as_select())
        .limit(per_page as i64)
        .offset(if page == 1 { 0 } else { (page - 1) * per_page } as i64)
        .load(connection)
        .expect("Error loading tasks");

    let count: i64 = tasks
        .select(count(title))
        .first(connection)
        .expect("Error loading tasks");

    (count, results)
}
