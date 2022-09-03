use crate::Settings;
use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;

pub async fn notify(text: &str) -> Result<()> {
    let notify_url = &Settings::get().slack_notify_url;
    if let Some(notify_url) = notify_url {
        let mut body = HashMap::new();
        body.insert("text", text.to_string());
        Client::new().post(notify_url).json(&body).send().await?;
    }

    Ok(())
}
