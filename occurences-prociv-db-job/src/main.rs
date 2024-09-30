use std::ops::DerefMut;
use std::str::FromStr;

use anyhow::bail;
use clap::Parser;
use futures::StreamExt;
use geozero::wkb::Encode;
use occurences_prociv_db_job::occurrences::convert::ConversionError;
use occurences_prociv_db_job::occurrences::occurrence::v1::ListOccurrencesRequest;
use occurences_prociv_db_job::occurrences::occurrence::v1::{
    occurrences_service_client::OccurrencesServiceClient, ListOccurrencesResponse,
};
use occurences_prociv_db_job::{AnepcId, InsertOccurrence};
use sentry::types::Dsn;
use sqlx::{query, query_scalar, PgPool, Postgres, Transaction};
use tokio::task::JoinSet;
use tonic::Status;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
        occurrences_service_url,
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

    let mut client = OccurrencesServiceClient::connect(occurrences_service_url).await?;

    let mut response = client
        .list_occurrences(ListOccurrencesRequest {})
        .await?
        .into_inner();

    let mut join_set: JoinSet<anyhow::Result<()>> = JoinSet::new();

    while let Some(response) = response.next().await {
        let sqlx_pool = sqlx_pool.clone();
        join_set.spawn(async move {
            let mut tx = sqlx_pool.begin().await?;
            process_response(&mut tx, response).await.inspect_err(|err| {
                match &err {
                    JobError::RepeatedOccurrence { .. } => todo!(),
                    JobError::DatabaseError(_) => todo!(),
                    JobError::GrpcError(_) => todo!(),
                    JobError::ConversionError(_) => todo!(),
                    JobError::UnpopulatedOccurrence => todo!(),
                }
            })?;
            tx.commit().await?;
            Ok(())
        });
    }
    while let Some(response) = response.next().await {
        if let Err(err) = response {
            tracing::error!("{err}");
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum JobError {
    #[error("repeated data: {} at {}", occurrence.anepc_id, occurrence.data_generated_at)]
    RepeatedOccurrence { occurrence: InsertOccurrence },
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("grpc error: {0}")]
    GrpcError(#[from] tonic::Status),
    #[error("conversion error: {0}")]
    ConversionError(#[from] ConversionError),
    #[error("unpopulated occurrence")]
    UnpopulatedOccurrence,
}

async fn process_response(
    tx: &mut Transaction<'_, Postgres>,
    response: Result<ListOccurrencesResponse, Status>,
) -> Result<(), JobError> {
    let occurrence = occurences_prociv_db_job::InsertOccurrence::try_from(
        response?
            .occurrence
            .ok_or(JobError::UnpopulatedOccurrence)?,
    )?;

    let exists = query_scalar!(
        r#"
            SELECT EXISTS (
                SELECT 1
                FROM occurrences
                WHERE "anepc_id" = $1 AND "data_generated_at" = $2
            ) AS "exists!: _";
        "#,
        occurrence.anepc_id as _,
        occurrence.data_generated_at,
    )
    .fetch_one(tx.deref_mut())
    .await?;

    if exists {
        return Err(JobError::RepeatedOccurrence { occurrence });
    }

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
    .execute(tx.deref_mut())
    .await?;

    Ok(())
}
