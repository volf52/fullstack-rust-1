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
