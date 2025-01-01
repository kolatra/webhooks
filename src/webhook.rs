use serde::{Deserialize, Serialize};
use webhook::client::WebhookClient;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WebhookAlert {
    // the url of the webhook
    url: String,
    // an ID for this webhook to find it later
    nickname: String,
    // the last username on the webhook
    username: String,
    // a user to mention in the message
    user_id: i64
}

impl WebhookAlert {
    pub(crate) async fn send_alert(&self, alert: &str, mention: bool) {
        let client = WebhookClient::new(&self.url);
        let content = if mention {
            String::from(format!("<@{}> {}", self.user_id, alert))
        } else {
            String::from(alert)
        };

        let res = client.send(|message| message
            .username(&self.username)
            .content(&content)).await;

        if let Err(e) = res {
            eprintln!("Error sending alert: {:?}", e);
        }
    }

    pub(crate) fn get_nickname(&self) -> &str { &self.nickname }
    pub(crate) fn set_username(&mut self, username: &str) { self.username = username.to_owned() }
}
