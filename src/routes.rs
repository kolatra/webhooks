use actix_web::{get, web, Responder};

use crate::AppState;

#[get("/new/{url}/{name}/{username}")]
async fn add_webhook(input: web::Path<(String, String, String)>, state: web::Data<AppState>) -> impl Responder {
    // this is the only one that writes to the RwLock, the rest read
    format!("{} {}", input.0, input.1)
}

#[get("/send/{nickname}/{content}")]
async fn send_alert(input: web::Path<(String, String)>, state: web::Data<AppState>) -> impl Responder {
    let (webhook_name, content) = input.into_inner();
    let list = state.webhooks.read().await;
    let wh = list.iter().filter(|w| w.get_nickname().eq(&webhook_name)).next();

    match wh {
        Some(webhook) => {
            webhook.send_alert(&content).await;
            "Command received successfully!"
        },
        None => "Requested webhook was not found!"
    }
}