use derive_more::Display;
use sqlx::query::Map;
use sqlx::IntoArguments;
use std::fmt::Debug;
use std::str::FromStr;
use strum::IntoEnumIterator;
use thiserror::Error;

#[derive(Error, Debug, Display)]
#[display("no variant for: {}", _0)]
pub struct NoVariantFori32(pub i32);

/// Lookup record. Represents the row of a lookup table
#[derive(Debug)]
pub struct LookupRecord {
    pub id: i32,
    pub value: String,
}

/// Lookup table. This should be implemented for types that directly map to the rows of a lookup table. Most likely enums
pub trait LookupTable<'a, Database: sqlx::Database>:
    PartialEq
    + Eq
    + Debug
    + IntoEnumIterator
    + Into<&'static str>
    + TryFrom<&'static str>
    + FromStr
    + Clone
    + Copy
    + TryFrom<i32>
    + Into<i32>
    + 'a
{
    const TABLE_NAME: &'static str;
    fn fetch_all_records(
    ) -> Map<'a, Database, impl FnMut(Database::Row) -> sqlx::Result<LookupRecord> + Send, impl IntoArguments<'a, Database>>;
    fn fetch_record_by_id(
        id: i32,
    ) -> Map<'a, Database, impl FnMut(Database::Row) -> sqlx::Result<LookupRecord> + Send, impl IntoArguments<'a, Database>>;
    fn fetch_record_by_value(
        value: &str,
    ) -> Map<'a, Database, impl FnMut(Database::Row) -> sqlx::Result<LookupRecord> + Send, impl IntoArguments<'a, Database>>;
}

pub use database_macros::*;
