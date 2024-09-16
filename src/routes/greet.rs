use actix_web::{get, web, Responder};

#[get("/greet/{id}")]
async fn greet_user(user_id: web::Path<u32>) -> impl Responder {
    format!("Hello, user with ID: {user_id}")
}