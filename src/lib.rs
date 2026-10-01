use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;
use serde::Deserialize;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
#[derive(Deserialize)]
struct FromData{
    email : String,
    name :  String,
}

async fn subcribe(_from : web::Form<FromData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subcribe))
    ).listen(listener)?
        .run();
    Ok(server)
}
