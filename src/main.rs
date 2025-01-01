use std::{fs::File, io::Write, sync::Arc};

use actix_web::{web, App, HttpServer};
use tokio::sync::RwLock;

mod cache;
mod routes;
mod webhook;

struct AppState {
    webhooks: Arc<RwLock<Vec<webhook::WebhookAlert>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // any config will go in .env
    dotenvy::dotenv().ok();
    println!("dotenv");

    let server_port: u16 = std::env::var("PORT")
        .unwrap()
        .parse()
        .unwrap();

    // load webhooks from json file
    // TODO: environment variable
    let cache = cache::JsonLoader::new(&std::env::var("CACHE").unwrap()).await?;
    let cache_ptr = Arc::new(RwLock::new(cache.loaded));

    // start command server
    println!("Starting web server");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                webhooks: cache_ptr.clone()
            }))
            .service(routes::add_webhook)
            .service(routes::send_alert)
    })
    .bind(("127.0.0.1", server_port))?
    .run()
    .await
}
