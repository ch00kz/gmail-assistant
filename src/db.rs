use sqlx::SqlitePool;

pub mod models;

pub async fn get_pool() -> sqlx::Result<SqlitePool, sqlx::Error> {
    SqlitePool::connect("sqlite://gmail-assistant.db").await
}
