use crate::db::repositories::TaskRepo;
use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use common::models::task::Task;

async fn fetch_task_by_id(Path(task_id): Path<String>) -> Json<Option<Task>> {
    let task = TaskRepo::get_task(task_id).await;

    Json(task)
}
async fn fetch_all_tasks() -> Json<Vec<Task>> {
    let tasks = TaskRepo::get_all_tasks().await;

    Json(tasks)
}

#[derive(serde::Deserialize)]
struct NewTask {
    user_id: String,
    task_type: String,
    src_file: String,
}

async fn insert_task(Json(task_data): Json<NewTask>) -> Json<String> {
    let task = Task::new(task_data.user_id, task_data.task_type, task_data.src_file);

    TaskRepo::put_task(&task).await;

    Json(task.task_id)
}

pub fn create_task_router() -> Router {
    Router::new()
        .route("/:task_id", get(fetch_task_by_id))
        .route("/", post(insert_task).get(fetch_all_tasks))
}
