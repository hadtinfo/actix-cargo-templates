use sqlx::mysql::MySqlPool;

pub mod db {
    use super::*;

    pub async fn create_pool() -> MySqlPool {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        MySqlPool::connect(&database_url).await.expect("Failed to create pool")
    }
}
