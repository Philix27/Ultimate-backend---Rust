mod routes;

use actix_web::web;
use routes::organization::org_routes_handler;
use routes::payments::payment_routes_handler;
use routes::transactions::transactions_routes_handler;
use routes::user::user_routes_handler;
use routes::utils::utils_routes_handler;
use routes::{auth::auth_routes_handler, notification::notification_routes_handler};

use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/notify").to(notify))
            .service(notification_routes_handler())
            .service(user_routes_handler())
            .service(transactions_routes_handler())
            .service(payment_routes_handler())
            .service(org_routes_handler())
            .service(auth_routes_handler())
            .service(utils_routes_handler())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix web")
}
async fn notify() -> impl Responder {
    HttpResponse::Ok().body("Notifier")
}
