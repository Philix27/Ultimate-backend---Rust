use actix_web::web;

async fn index() -> &'static str {
    "Hello, World!"
}

async fn user_info() -> &'static str {
    "User info"
}

pub fn transactions_routes_handler() -> actix_web::Scope {
    web::scope("/auth")
        .route("/", web::get().to(index))
        .route("/{user_id}", web::patch().to(user_info))
        .route("/{user_id}", web::delete().to(user_info))
}
