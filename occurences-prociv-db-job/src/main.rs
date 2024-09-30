use std::str::FromStr;

use clap::Parser;
use futures::StreamExt;
use geozero::wkb::Encode;
use sentry::types::Dsn;
use sqlx::{query,  PgPool};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use occurences_prociv_db_job::occurrences::portugal_reimagined::occurrences_service::v1::my_occurrences_service_client::MyOccurrencesServiceClient;
use occurences_prociv_db_job::occurrences::portugal_reimagined::occurrences_service::v1::ListOccurrencesRequest;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(short, long, env)]
    occurrences_service_url: String,
    #[clap(short, long, env)]
    sentry_dsn: String,

    #[clap(short, long, env)]
    connection_string: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let Args {
        occurrences_service_url: _,
        sentry_dsn,
        connection_string,
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

    let sqlx_pool: sqlx::Pool<sqlx::Postgres> = PgPool::connect(&connection_string).await.unwrap();

    let mut client = MyOccurrencesServiceClient::connect("http://[::1]:50051").await?;

    let mut response = client
        .list_occurrences(ListOccurrencesRequest {})
        .await?
        .into_inner();

    while let Some(response) = response.next().await {
        let response = match response {
            Ok(response) => response,
            Err(err) => {
                tracing::error!("error streaming next: {err}");
                continue;
            }
        };
        let occurrence = if let Some(occurrence) = response.occurrence {
            occurrence
        } else {
            tracing::error!("unpopulated occurrence");
            continue;
        };
        let occurrence = occurences_prociv_db_job::InsertOccurrence::try_from(occurrence);
        let occurrence = match occurrence {
            Ok(occurrence) => occurrence,
            Err(err) => {
                tracing::error!("error converting occurrence to DB model: {err}");
                continue;
            }
        };

        query!(
            r#"
            INSERT INTO occurrences (
                "location",
                "kind",
                "grouped_status",
                "occurrence_status",
                "crepc",
                "csrepc",
                "anepc_id",
                "number_of_operatives",
                "number_of_land_means",
                "number_of_water_means",
                "number_of_air_means",
                "start_date",
                "data_generated_at"
            )
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13);
            "#,
            Encode(occurrence.location) as _,
            occurrence.kind as _,
            occurrence.grouped_status as _,
            occurrence.occurrence_status as _,
            occurrence.crepc as _,
            occurrence.csrepc as _,
            occurrence.anepc_id as _,
            occurrence.number_of_operatives,
            occurrence.number_of_land_means,
            occurrence.number_of_water_means,
            occurrence.number_of_air_means,
            occurrence.start_date,
            occurrence.data_generated_at,
        )
        .execute(&sqlx_pool)
        .await
        .inspect_err(|err| tracing::error!("error inserting occurrence: {err}"))
        .ok();
    }
    Ok(())
}
