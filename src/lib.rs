use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use serde::Deserialize;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct UserForm {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<UserForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
