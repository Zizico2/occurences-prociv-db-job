use std::str::FromStr;

use clap::Parser;
use sentry::types::Dsn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(short, long, env)]
    occurrences_service_url: String,
    #[clap(short, long, env)]
    sentry_dsn: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let Args {
        occurrences_service_url: _,
        sentry_dsn,
    } = Args::parse();
    let _guard = sentry::init(sentry::ClientOptions {
        traces_sample_rate: 1.0,
        dsn: Some(Dsn::from_str(&sentry_dsn)?),
        ..Default::default()
    });
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(sentry_tracing::layer())
        .try_init()?;
    Ok(())
}
