use actix_web::web;

async fn index() -> &'static str {
    "Hello, World!"
}

async fn user_info() -> &'static str {
    "User info"
}

pub fn auth_routes_handler() -> actix_web::Scope {
    web::scope("/auth")
        .route("/auth", web::get().to(index))
        .route("/auth/{user_id}", web::patch().to(user_info))
        .route("/auth/{user_id}", web::delete().to(user_info))
}
