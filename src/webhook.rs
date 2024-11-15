use reqwest::blocking::Client;
use serde_json::json;
use std::error::Error;

pub fn send_webhook(url: &str, message: &str) -> Result<(), Box<dyn Error>> {
    // create webhook payload
    let payload = json!({
        "content":message
    });
    // create HTTP Client
    let client = Client::new();
    // send webhook
    let res = client.post(url)
        .json(&payload)
        .send()?;

    // return error if failed
    if !res.status().is_success() {
        eprintln!("Failed sending webhook to discord. Status: {}", res.status());
    }
    Ok(())
}
