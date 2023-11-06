use actix_web::{web, HttpResponse};
use service_config::env as Config;
use service_messenger::queue::producer;

async fn send_email_otp() -> &'static str {
    "Hello, World!"
}

async fn send_sms_otp() -> &'static str {
    "Send sms otp"
}
async fn verify_otp() -> &'static str {
    "Verify otp"
}
async fn send_email_notification() -> &'static str {
    "User info"
}
async fn send_sms_notification() -> &'static str {
    producer::AppProducer::new(Config::ThirdParties::get_values().kafka_host);
    "Sent successfully"
}
async fn index() -> &'static str {
    print!("Reached index notification");
    "Sent successfully"
}

pub fn notification_routes_handler() -> actix_web::Scope {
    web::scope("/notification")
        .route("/", web::get().to(index))
        .route("/send_email_otp", web::post().to(send_email_otp))
        .route("/send_sms_otp", web::post().to(send_sms_otp))
        .route("/verify_otp", web::post().to(verify_otp))
        .route(
            "/send_email_notification",
            web::post().to(send_email_notification),
        )
        .route(
            "/send_sms_notification",
            web::post().to(send_sms_notification),
        )

    // web::scope("/notifa")
    //     .service(web::get().to(index))
    //     .service(web::resource("/path2").to(|| HttpResponse::Ok()))
    //     .service(web::resource("/path3").to(|| HttpResponse::MethodNotAllowed()))
}
