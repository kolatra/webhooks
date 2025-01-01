use std::{fs::File, io::Write, sync::Arc};

use actix_web::{web, App, HttpServer};
use tokio::sync::RwLock;

mod cache;
mod routes;
mod webhook;

struct AppState {
    webhooks: Arc<RwLock<Vec<webhook::WebhookAlert>>>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    create_dev_webhook().await;

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

#[cfg(debug_assertions)]
async fn create_dev_webhook() {
    let output = vec![webhook::WebhookAlert {
        url: "https://discord.com/api/webhooks/1076360611757305868/F88jTeNbtjAjhmPCgq_4ATspyN4k2t2v8LQnszXyiGTzeYHpPOf2slrWwX6lktM0YdXT".to_string(),
        nickname: "dev".to_string(),
        username: "Persefone".to_string(),
    }];

    let mut file = File::create("./webhooks.json").expect("could not create");

    let json_string = serde_json::to_string_pretty(&output).unwrap();
    file.write_all(json_string.as_bytes()).expect("could not write");
}
