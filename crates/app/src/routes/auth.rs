use actix_web::web;

async fn send_email_otp() -> &'static str {
    "Hello, World!"
}

async fn send_phone_otp() -> &'static str {
    "User info"
}

pub fn auth_routes_handler() -> actix_web::Scope {
    web::scope("/auth")
        .route("/", web::get().to(send_email_otp))
        .route("/{user_id}", web::patch().to(send_phone_otp))
}
