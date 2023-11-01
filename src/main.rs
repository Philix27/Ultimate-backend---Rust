mod routes;
use routes::auth::auth_routes_handler;
use routes::ledger::get_task;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/notification").to(index))
            .service(web::resource("/auth").to(index))
            .service(web::resource("/chat").to(index))
            .service(web::resource("/users/{username}").to(user_info))
            .service(auth_routes_handler())
            .service(get_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn user_info(path: web::Path<(String,)>) -> impl Responder {
    let username = &path.0;
    HttpResponse::Ok().body(format!("User info for: {}", username))
}
