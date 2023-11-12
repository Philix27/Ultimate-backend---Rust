use actix_web::dev::HttpServiceFactory;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use actix_web::{web, Scope};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 9020))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Notification service")
}
