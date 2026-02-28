use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserForm {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<UserForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
