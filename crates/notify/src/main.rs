use actix_web::web;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 9020))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix web")
}
async fn notify() -> impl Responder {
    HttpResponse::Ok().body("Notifier")
}
