use crate::shared::{repositories::tasks as repository, structs::application::ListResponse};

use super::response_dto::TaskResponseDTO;

pub fn execute(per_page: usize, page: usize) -> ListResponse<TaskResponseDTO> {
    let mut tasks: Vec<TaskResponseDTO> = Vec::new();
    let (total_tasks, results) = repository::find(per_page, page);

    for task in results {
        tasks.push(TaskResponseDTO::new(task))
    }

    ListResponse {
        per_page,
        total_page: if total_tasks as usize <= per_page {
            1
        } else {
            total_tasks as usize / per_page
        },
        items: tasks,
    }
}
