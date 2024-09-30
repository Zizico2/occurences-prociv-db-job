use chrono::Utc;
use derive_more::Display;
use lookup_tables::{Crepc, Csrepc, GroupedStatus, OccurrenceKind, OccurrenceStatus};

pub mod lookup_tables;
pub mod occurrences;

#[derive(Debug)]
pub struct InsertOccurrence {
    pub location: geo_types::Geometry<f64>,
    pub kind: OccurrenceKind,
    pub grouped_status: GroupedStatus,
    pub occurrence_status: OccurrenceStatus,
    pub crepc: Crepc,
    pub csrepc: Csrepc,
    pub anepc_id: AnepcId,
    pub number_of_operatives: i32,
    pub number_of_land_means: i32,
    pub number_of_water_means: i32,
    pub number_of_air_means: i32,
    pub data_generated_at: chrono::DateTime<Utc>,
    pub start_date: chrono::DateTime<Utc>,
}

#[derive(Debug, sqlx::Type, Clone, Copy)]
#[sqlx(transparent)]
pub struct OccurrenceId(pub i32);

#[derive(Debug, sqlx::Type, Clone, Display)]
#[sqlx(transparent)]
pub struct AnepcId(pub String);
