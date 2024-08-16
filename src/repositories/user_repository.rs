use sqlx::MySqlPool;
use crate::models::user::User;

pub async fn fetch_all_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(pool)
        .await
}

pub async fn insert_user(pool: &MySqlPool, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(&user.name)
        .bind(&user.email)
        .execute(pool)
        .await?;
    Ok(())
}

// Các hàm update_user và delete_user cũng được đặt ở đây.
