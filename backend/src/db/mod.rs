use common::models::task::Task;
use sqlx::{migrate::Migrator, Pool, Sqlite};
use tokio::sync::OnceCell;

static MIGRATOR: Migrator = sqlx::migrate!();

static POOL: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

pub async fn init_db_and_migrate() {
    dotenvy::dotenv().unwrap_or_default();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&db_url)
        .await
        .unwrap();
    MIGRATOR.run(&pool).await.unwrap();

    POOL.set(pool).unwrap();
}

pub mod repositories;

pub async fn seed_db() {
    let user = "test_user".to_string();
    let task_type = "test_task".to_string();

    let src_files = [
        "/test/a".to_string(),
        "/test/b".to_string(),
        "/test/c".to_string(),
    ];

    for src_file in src_files {
        let task = Task::new(user.clone(), task_type.clone(), src_file);

        repositories::TaskRepo::put_task(&task).await;
        log::info!("Inserted task {}", task.task_id);
    }
}
