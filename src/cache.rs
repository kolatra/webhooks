// the json cache of webhooks

use crate::webhook::WebhookAlert;

#[derive(Debug, Clone)]
pub struct JsonLoader {
    pub loaded: Vec<WebhookAlert>
}

impl JsonLoader {
    pub fn new(file_input: &str) -> std::io::Result<Self> {
        let raw = std::fs::read_to_string(file_input)?;
        let loaded: Vec<WebhookAlert> = serde_json::from_str(&raw)?;

        Ok(JsonLoader {
            loaded
        })
    }
}
