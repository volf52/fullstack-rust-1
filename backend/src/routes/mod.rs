mod health;
mod task;

pub fn create_api_router() -> axum::Router {
    let health_router = health::create_health_router();
    let task_router = task::create_task_router();

    axum::Router::new()
        .nest("/health", health_router)
        .nest("/task", task_router)
}
