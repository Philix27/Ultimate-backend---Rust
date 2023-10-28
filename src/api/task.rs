// #[allow(dead_code)]
use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    middleware::Logger,
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use serde::{Deserialize, Serialize};

// use derive_more::{Display};
#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_global_id: String,
}

#[get("/task/{task_global_id}")]
pub async fn get_task(task_identifier: Path<TaskIdentifier>, body: Json<String>) -> Json<String> {
    Json("Hello, from get task".to_owned())
}
