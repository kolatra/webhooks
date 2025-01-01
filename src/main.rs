use clap::Parser;

mod cache;
mod webhook;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long)]
    wh_name: String,
    #[arg(short, long)]
    content: String,
    #[arg(short, long)]
    username: Option<String>,
    #[arg(short, long)]
    ping: Option<bool>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let args = dbg!(args);

    // any config will go in .env
    dotenvy::dotenv().ok();
    println!("dotenv");

    // load webhooks from json file
    let mut cache = cache::JsonLoader::new(&std::env::var("CACHE").unwrap())?;

    let wh_opt = cache.loaded.iter_mut().filter(|wh| wh.get_nickname().eq(&args.wh_name)).next();

    match wh_opt {
        Some(webhook) => {
            if let Some(user) = &args.username {
                webhook.set_username(user);
            }
            webhook.send_alert(&args.content, args.ping.is_some()).await;
        },
        None => anyhow::bail!("Webhook was not found!")
    }

    Ok(())
}
