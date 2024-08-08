use crate::shared::{
    dtos::response::ResponseTaskDTO, repositories::tasks as repository,
    structs::application::ListResponse,
};

pub fn execute(per_page: usize, page: usize) -> ListResponse<ResponseTaskDTO> {
    let mut tasks: Vec<ResponseTaskDTO> = Vec::new();
    let (total_tasks, results) = repository::find(per_page, page);

    for task in results {
        tasks.push(ResponseTaskDTO::new(task))
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
