use actix_web::{web, HttpResponse, Scope};
use service_config::env as Config;
use service_messenger::queue::producer::{AppKafkaTopics, AppProducer};

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
    AppProducer::send_data(
        &AppProducer::new(),
        "input".to_owned(),
        AppKafkaTopics::Notification,
    )
    .await;
    "Sent to queue"
}
async fn send_sms_notification() -> &'static str {
    "Sent successfully"
}
async fn index() -> &'static str {
    print!("Reached index notification");
    "Sent successfully"
}

pub fn notification_routes_handler() -> Scope {
    web::scope("/notification")
        .route("", web::get().to(index))
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
}

// fn notify_route() -> Scope {
//     web::scope("/notify")
//         .route("holo", web::get().to(notify))
//         .route("men", web::get().to(notify_men))
// }
