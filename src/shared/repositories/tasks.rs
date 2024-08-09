use chrono::Utc;
use diesel::{dsl::count, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    schema::tasks::{self, dsl::*},
    shared::{
        entities::tasks::{NewTask, Task, UpdateTask},
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

pub fn find_by_id(task_id: &str) -> Option<Task> {
    let connection = &mut establish_connection();

    match tasks
        .find(uuid::Uuid::parse_str(task_id).unwrap())
        .filter(deleted_at.is_null())
        .select(Task::as_select())
        .first(connection)
    {
        Err(_) => None,
        Ok(value) => Some(value),
    }
}

pub fn delete_one(task_id: &str) -> bool {
    let connection = &mut establish_connection();

    diesel::update(tasks.find(uuid::Uuid::parse_str(task_id).unwrap()))
        .set(deleted_at.eq(Some(Utc::now().naive_utc())))
        .execute(connection)
        .is_ok()
}

pub fn update_one(task_id: &str, payload: UpdateTask) -> Result<Task, ()> {
    let connection = &mut establish_connection();

    match diesel::update(tasks.find(uuid::Uuid::parse_str(task_id).unwrap()))
        .set((
            title.eq(payload.title),
            description.eq(payload.description),
            checked.eq(payload.checked),
            updated_at.eq(Utc::now().naive_utc()),
        ))
        .returning(Task::as_returning())
        .execute(connection)
    {
        Ok(_) => {
            if let Some(task) = find_by_id(task_id) {
                Ok(task)
            } else {
                eprintln!("Error: Some error occurred!");
                Err(())
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            Err(())
        }
    }
}
