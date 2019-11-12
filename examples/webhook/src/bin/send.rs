use dbl::types::{Webhook, WebhookType};
use reqwest::header::AUTHORIZATION;

#[tokio::main]
async fn main() {
    let url = "http://localhost:3030/dbl/webhook";
    let secret = "mywebhook";

    let data = Webhook {
        bot: 1234.into(),
        user: 2345.into(),
        kind: WebhookType::Test,
        is_weekend: false,
        query: None,
    };

    let resp = reqwest::Client::new()
        .post(url)
        .header(AUTHORIZATION, secret)
        .json(&data)
        .send()
        .await;

    if let Err(e) = resp.map(|resp| resp.error_for_status()) {
        eprintln!("{}", e);
    }
}
