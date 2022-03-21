use clap::StructOpt;
use hookspammer::{
    create_client, send_random_events_for_event_type, send_random_events_with_random_event_types,
    Args,
};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let client = create_client();
    let url = format!("{}/webhook/github", args.server_ip);
    let secret = args.webhook_secret.as_ref();

    if let Some(event_type) = args.event_type {
        send_random_events_for_event_type(&client, secret, &url, event_type, args.count).await?;
    } else {
        send_random_events_with_random_event_types(&client, secret, &url, args.count).await?;
    }

    Ok(())
}
