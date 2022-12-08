use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Debug, Serialize)]
enum HealthStatus {
    Ok,
}

#[derive(Debug, Serialize)]
struct Health {
    status: HealthStatus,
}

impl Health {
    fn ok() -> Self {
        Self {
            status: HealthStatus::Ok,
        }
    }
}

async fn health() -> Json<Health> {
    Json(Health::ok())
}

pub fn create_health_router() -> Router {
    Router::new().route("/", get(health))
}
