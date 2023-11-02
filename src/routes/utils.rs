use actix_web::web;

async fn get_country() -> &'static str {
    "Hello, World!"
}

async fn get_states() -> &'static str {
    "User info"
}
async fn get_state() -> &'static str {
    "User info"
}

pub fn utils_routes_handler() -> actix_web::Scope {
    web::route("/utils/country", web::get().to(get_country))
        .route("/utils/states", web::get().to(get_states))
        .route("/utils/state/{user_id}", web::patch().to(get_state))
}
