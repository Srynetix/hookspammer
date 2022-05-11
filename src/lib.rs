pub mod crypto;
pub mod models;

use std::time::Duration;

use crate::crypto::compute_signature;
use crate::models::common::{EventType, GitBackend};
use clap::Parser;
use fake::{Dummy, Fake, Faker};
use models::checks::CheckSuiteEvent;
use models::issues::IssueCommentEvent;
use models::ping::PingEvent;
use models::pulls::PullRequestEvent;
use models::push::PushEvent;
use models::reviews::ReviewEvent;
use models::unknown::UnknownEvent;
use reqwest::{Client, Response};
use serde::Serialize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Webhook secret (disabled as default)
    #[clap(short('s'), long)]
    pub webhook_secret: Option<String>,

    /// Git hosting backend
    #[clap(short('b'), long, default_value = "github")]
    pub backend: GitBackend,

    /// Server IP
    #[clap(short('i'), long, default_value = "http://127.0.0.1:3000")]
    pub server_ip: String,

    /// Webhook URL
    #[clap(short('u'), long, default_value = "/webhook")]
    pub webhook_url: String,

    /// Event type
    #[clap(short('e'), long)]
    pub event_type: Option<EventType>,

    /// Event count
    #[clap(short('c'), long, default_value = "1")]
    pub count: usize,
}

async fn send_random_event<E: Dummy<Faker> + Serialize>(
    client: &Client,
    secret: Option<&String>,
    url: &str,
    event_type: EventType,
) -> color_eyre::Result<Response> {
    let data = Faker.fake::<E>();
    let serialized_data = serde_json::to_string(&data)?;

    let mut builder = client
        .post(url)
        .header("X-GitHub-Event", event_type.to_string())
        .header("Content-Type", "application/json");

    if let Some(s) = secret {
        let signature = compute_signature(serialized_data.as_bytes(), s);
        builder = builder.header("X-Hub-Signature-256", format!("sha256={}", signature));
    }

    let response = builder.body(serialized_data).send().await?;
    Ok(response)
}

pub fn create_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(10))
        .user_agent("GitHub-Hookshot/1.0.0")
        .build()
        .unwrap()
}

pub async fn send_random_event_for_event_type(
    client: &Client,
    secret: Option<&String>,
    url: &str,
    event_type: EventType,
) -> color_eyre::Result<Response> {
    match event_type {
        EventType::CheckSuite => {
            send_random_event::<CheckSuiteEvent>(client, secret, url, event_type).await
        }
        EventType::IssueComment => {
            send_random_event::<IssueCommentEvent>(client, secret, url, event_type).await
        }
        EventType::Ping => send_random_event::<PingEvent>(client, secret, url, event_type).await,
        EventType::Push => send_random_event::<PushEvent>(client, secret, url, event_type).await,
        EventType::PullRequest => {
            send_random_event::<PullRequestEvent>(client, secret, url, event_type).await
        }
        EventType::Review => {
            send_random_event::<ReviewEvent>(client, secret, url, event_type).await
        }
        EventType::Unknown => {
            send_random_event::<UnknownEvent>(client, secret, url, event_type).await
        }
    }
}

pub async fn send_random_event_with_random_event_type(
    client: &Client,
    secret: Option<&String>,
    url: &str,
) -> color_eyre::Result<Response> {
    send_random_event_for_event_type(client, secret, url, Faker.fake()).await
}

pub async fn send_random_events_for_event_type(
    client: &Client,
    secret: Option<&String>,
    url: &str,
    event_type: EventType,
    count: usize,
) -> color_eyre::Result<()> {
    for _ in 0..count {
        let response = send_random_event_for_event_type(client, secret, url, event_type).await;
        handle_response(response).await?;
    }

    Ok(())
}

pub async fn send_random_events_with_random_event_types(
    client: &Client,
    secret: Option<&String>,
    url: &str,
    count: usize,
) -> color_eyre::Result<()> {
    for _ in 0..count {
        let response = send_random_event_with_random_event_type(client, secret, url).await;
        handle_response(response).await?;
    }

    Ok(())
}

async fn handle_response(response: color_eyre::Result<Response>) -> color_eyre::Result<()> {
    match response {
        Err(e) => {
            eprintln!("[ERROR] {}", e);
        }
        Ok(resp) => {
            let status = resp.status();
            let code = status.as_u16();
            println!("[OK/{code}] {}", resp.text().await?);
        }
    }

    Ok(())
}
