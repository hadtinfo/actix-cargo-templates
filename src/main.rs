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
