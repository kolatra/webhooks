use serde::{Deserialize, Serialize};
use webhook::client::WebhookClient;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WebhookAlert {
    // the url of the webhook
    pub url: String,
    // an ID for this webhook to find it later
    pub nickname: String,
    // the last username on the webhook
    pub username: String,
}

impl WebhookAlert {
    pub(crate) async fn send_alert(&self, alert: &str) {
        let client = WebhookClient::new(&self.url);

        let res = client.send(|message| message
            .username(&self.username)
            .content(alert)).await;

        if let Err(e) = res {
            eprintln!("Error sending alert: {:?}", e);
        }
    }

    pub(crate) fn get_nickname(&self) -> &str { &self.nickname }
    pub(crate) fn get_username(&self) -> &str { &self.username }
    pub(crate) fn set_username(&mut self, username: String) { self.username = username }
}
