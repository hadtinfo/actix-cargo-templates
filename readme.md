Khi xây dựng một dự án Rust sử dụng Actix Web framework và tương tác với MySQL, bạn sẽ cần tổ chức cấu trúc dự án một cách hợp lý để dễ dàng quản lý và phát triển. Dưới đây là một cấu trúc dự án mẫu, phù hợp cho các ứng dụng web có tương tác với cơ sở dữ liệu:

lua
Copy code
my_actix_project/
│
├── Cargo.toml
├── .env
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── user_handlers.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   ├── repositories/
│   │   ├── mod.rs
│   │   ├── user_repository.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── user_service.rs
│   └── schema.rs
└── migrations/
Chi tiết các thành phần
1. Cargo.toml
Khai báo các dependencies của dự án, bao gồm Actix Web, SQLx, và các thư viện khác.
toml
Copy code
[package]
name = "my_actix_project"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
sqlx = { version = "0.6", features = ["mysql", "runtime-actix-native-tls"] }
dotenv = "0.15"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
2. .env
Lưu trữ các biến môi trường như thông tin kết nối cơ sở dữ liệu.
dotenv
Copy code
DATABASE_URL=mysql://username:password@localhost/database_name
3. src/main.rs
Điểm khởi đầu của ứng dụng. Tại đây bạn khởi tạo Actix Web server và cấu hình các route.
rust
Copy code
mod config;
mod handlers;
mod models;
mod repositories;
mod services;
mod schema;

use actix_web::{web, App, HttpServer};
use config::db::create_pool;
use handlers::user_handlers::{get_users, add_user, update_user, delete_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let pool = create_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/users")
                .route("", web::get().to(get_users))
                .route("", web::post().to(add_user))
                .route("/{id}", web::put().to(update_user))
                .route("/{id}", web::delete().to(delete_user))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
4. src/config.rs
Cấu hình kết nối cơ sở dữ liệu.
rust
Copy code
use sqlx::mysql::MySqlPool;

pub mod db {
    use super::*;

    pub async fn create_pool() -> MySqlPool {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        MySqlPool::connect(&database_url).await.expect("Failed to create pool")
    }
}
5. src/handlers/user_handlers.rs
Xử lý các yêu cầu HTTP liên quan đến người dùng (users).
rust
Copy code
use actix_web::{web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::models::user::User;
use crate::repositories::user_repository;

pub async fn get_users(pool: web::Data<MySqlPool>) -> impl Responder {
    let users = user_repository::fetch_all_users(pool.get_ref()).await.unwrap();
    HttpResponse::Ok().json(users)
}

pub async fn add_user(pool: web::Data<MySqlPool>, user: web::Json<User>) -> impl Responder {
    user_repository::insert_user(pool.get_ref(), &user).await.unwrap();
    HttpResponse::Ok().body("User added")
}

// update_user và delete_user tương tự, nhưng với các thao tác UPDATE và DELETE.
6. src/models/user.rs
Định nghĩa các cấu trúc dữ liệu (model) tương ứng với các bảng trong cơ sở dữ liệu.
rust
Copy code
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}
7. src/repositories/user_repository.rs
Xử lý các tương tác trực tiếp với cơ sở dữ liệu.
rust
Copy code
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
8. src/services/
Thư mục này có thể chứa các logic nghiệp vụ phức tạp hơn liên quan đến người dùng hoặc các thành phần khác.
9. src/schema.rs
Tùy thuộc vào cách tổ chức, file này có thể chứa các macro của Diesel hoặc các định nghĩa schema khác nếu bạn sử dụng Diesel.
10. migrations/
Nếu bạn sử dụng Diesel, thư mục này sẽ chứa các migration cho cơ sở dữ liệu.
Tổng kết
Cấu trúc trên giúp tổ chức mã nguồn theo cách dễ quản lý và bảo trì, đồng thời tách biệt rõ ràng các trách nhiệm giữa việc xử lý HTTP, logic nghiệp vụ, và tương tác cơ sở dữ liệu. Mỗi thư mục và file có một nhiệm vụ rõ ràng, giúp bạn mở rộng và phát triển dự án một cách dễ dàng hơn.