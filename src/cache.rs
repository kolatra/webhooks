// the json cache of webhooks

use crate::webhook::WebhookAlert;

#[derive(Debug, Clone)]
pub struct JsonLoader {
    pub loaded: Vec<WebhookAlert>
}

impl JsonLoader {
    pub async fn new(file_input: &str) -> tokio::io::Result<Self> {
        let raw = tokio::fs::read_to_string(file_input).await?;
        let loaded: Vec<WebhookAlert> = serde_json::from_str(&raw)?;

        Ok(JsonLoader {
            loaded
        })
    }
}
