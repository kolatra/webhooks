use clap::Parser;

mod cache;
mod webhook;

#[derive(clap::Parser, Debug)]
struct Args {
    wh_name: String,
    content: String,
    username: Option<String>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let args = dbg!(args);

    // any config will go in .env
    dotenvy::dotenv().ok();
    println!("dotenv");

    // load webhooks from json file
    let cache = cache::JsonLoader::new(&std::env::var("CACHE").unwrap())?;

    for mut wh in cache.loaded {
        if wh.get_nickname().eq(&args.wh_name) {
            if let Some(user) = &args.username {
                wh.set_username(user.clone());
            }

            wh.send_alert(&args.content).await;
        }
    }

    Ok(())
}
