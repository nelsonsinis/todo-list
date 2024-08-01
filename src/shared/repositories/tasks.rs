use diesel::{dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    schema::tasks::{self, dsl::*},
    shared::{
        entities::tasks::{NewTask, Task},
        utils::database::establish_connection,
    },
};

pub fn find(per_page: usize, page: usize) -> (i64, Vec<Task>) {
    let connection = &mut establish_connection();

    let results: Vec<Task> = tasks
        .filter(deleted_at.is_null())
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

pub fn create(body: NewTask) -> Task {
    let connection = &mut establish_connection();

    diesel::insert_into(tasks::table)
        .values(&body)
        .returning(Task::as_returning())
        .get_result(connection)
        .expect("Error saving new task")
}
