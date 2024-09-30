use chrono::{DateTime, Utc};

use prost_types::Timestamp;

use crate::occurrences::foundation::v1::Coordinates;
use crate::{lookup_tables, AnepcId};

use super::occurrence::v1::{GroupedStatus, Occurrence, OccurrenceStatus};
use super::spatial_planning::v1::{Crepc, Csrepc};

mod code_map;

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("unknown")]
    Unknown,
}

impl From<Coordinates> for geo_types::geometry::Coord {
    fn from(value: Coordinates) -> Self {
        Self {
            x: value.longitude,
            y: value.latitude,
        }
    }
}

impl From<Coordinates> for geo_types::geometry::Point {
    fn from(value: Coordinates) -> Self {
        geo_types::geometry::Coord::from(value).into()
    }
}

use std::convert::TryFrom;

impl TryFrom<Crepc> for lookup_tables::Crepc {
    type Error = ConversionError;

    fn try_from(value: Crepc) -> Result<Self, ConversionError> {
        let res = match value {
            Crepc::C1 => Self::C1,
            Crepc::C2 => Self::C2,
            Crepc::C3 => Self::C3,
            Crepc::C4 => Self::C4,
            Crepc::C5 => Self::C5,
            Crepc::Unspecified => return Err(ConversionError::Unknown),
        };
        Ok(res)
    }
}

impl TryFrom<Csrepc> for lookup_tables::Csrepc {
    type Error = ConversionError;

    fn try_from(value: Csrepc) -> Result<Self, ConversionError> {
        let res = match value {
            Csrepc::C1 => Self::C1,
            Csrepc::C2 => Self::C2,
            Csrepc::C3 => Self::C3,
            Csrepc::C4 => Self::C4,
            Csrepc::C5 => Self::C5,
            Csrepc::C6 => Self::C6,
            Csrepc::C7 => Self::C7,
            Csrepc::C8 => Self::C8,
            Csrepc::C9 => Self::C9,
            Csrepc::C10 => Self::C10,
            Csrepc::C11 => Self::C11,
            Csrepc::C12 => Self::C12,
            Csrepc::C13 => Self::C13,
            Csrepc::C14 => Self::C14,
            Csrepc::C15 => Self::C15,
            Csrepc::C16 => Self::C16,
            Csrepc::C17 => Self::C17,
            Csrepc::C18 => Self::C18,
            Csrepc::C19 => Self::C19,
            Csrepc::C20 => Self::C20,
            Csrepc::C21 => Self::C21,
            Csrepc::C22 => Self::C22,
            Csrepc::C23 => Self::C23,
            Csrepc::C24 => Self::C24,
            Csrepc::Unspecified => return Err(ConversionError::Unknown),
        };
        Ok(res)
    }
}

impl TryFrom<GroupedStatus> for lookup_tables::GroupedStatus {
    type Error = ConversionError;

    fn try_from(value: GroupedStatus) -> Result<Self, ConversionError> {
        let res = match value {
            GroupedStatus::Active => Self::Active,
            GroupedStatus::Concluding => Self::Concluding,
            GroupedStatus::Dispatching => Self::Dispatching,
            GroupedStatus::Resolving => Self::Resolving,
            GroupedStatus::Unspecified => return Err(ConversionError::Unknown),
        };
        Ok(res)
    }
}

impl TryFrom<OccurrenceStatus> for lookup_tables::OccurrenceStatus {
    type Error = ConversionError;

    fn try_from(value: OccurrenceStatus) -> Result<Self, ConversionError> {
        let res = match value {
            OccurrenceStatus::Active => Self::Active,
            OccurrenceStatus::FirstDispatch => Self::FirstDispatch,
            OccurrenceStatus::Dispatch => Self::Dispatching,
            OccurrenceStatus::Concluding => Self::Concluding,
            OccurrenceStatus::Resolving => Self::Resolving,
            OccurrenceStatus::Monitoring => Self::Monitoring,
            OccurrenceStatus::SiteArrival => Self::SiteArrival,
            OccurrenceStatus::Unspecified => return Err(ConversionError::Unknown),
        };
        Ok(res)
    }
}

impl TryFrom<Occurrence> for crate::InsertOccurrence {
    type Error = ConversionError;

    fn try_from(occurrence: Occurrence) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: occurrence
                .kind
                .ok_or(ConversionError::Unknown)?
                .try_into()?,
            grouped_status: occurrence.grouped_status().try_into()?,
            occurrence_status: occurrence.occurence_status().try_into()?,
            crepc: occurrence.crepc().try_into()?,
            csrepc: occurrence.csrepc().try_into()?,
            anepc_id: AnepcId(occurrence.anepc_number.to_string()),
            number_of_operatives: occurrence
                .number_of_operatives
                .try_into()
                .map_err(|_| ConversionError::Unknown)?,
            number_of_land_means: occurrence
                .number_of_land_means
                .try_into()
                .map_err(|_| ConversionError::Unknown)?,
            number_of_water_means: occurrence
                .number_of_water_means
                .try_into()
                .map_err(|_| ConversionError::Unknown)?,
            number_of_air_means: occurrence
                .number_of_air_means
                .try_into()
                .map_err(|_| ConversionError::Unknown)?,

            data_generated_at: DateTime::<Utc>::try_from_proto(
                occurrence
                    .data_generated_at
                    .ok_or(ConversionError::Unknown)?,
            )?,
            start_date: DateTime::<Utc>::try_from_proto(
                occurrence.start_date.ok_or(ConversionError::Unknown)?,
            )?,
            location: occurrence
                .location
                .map(geo_types::geometry::Point::from)
                .ok_or(ConversionError::Unknown)?
                .into(),
        })
    }
}

pub trait TimestampExt {
    fn from_chrono(date_time: DateTime<Utc>) -> Result<Self, ConversionError>
    where
        Self: Sized;
}

impl TimestampExt for Timestamp {
    fn from_chrono(date_time: DateTime<Utc>) -> Result<Self, ConversionError> {
        let seconds = date_time.timestamp();
        let nanoseconds = date_time.timestamp_subsec_nanos();

        let nanos = nanoseconds
            .try_into()
            .map_err(|_| ConversionError::Unknown)?;

        Ok(Self { seconds, nanos })
    }
}

pub trait DateTimeExt {
    fn try_from_proto(timestamp: Timestamp) -> Result<Self, ConversionError>
    where
        Self: Sized;
}

impl DateTimeExt for DateTime<Utc> {
    fn try_from_proto(timestamp: Timestamp) -> Result<Self, ConversionError> {
        Self::from_timestamp(
            timestamp.seconds,
            timestamp
                .nanos
                .try_into()
                .map_err(|_| ConversionError::Unknown)?,
        )
        .ok_or(ConversionError::Unknown)
    }
}
