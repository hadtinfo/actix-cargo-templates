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
