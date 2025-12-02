use axum::{Json};
use crate::service::hello_service::HelloService;

pub async fn hello_handler() -> Json<String> {
    let service = HelloService {};
    let result = service.say_hello();
    Json(result)
}
