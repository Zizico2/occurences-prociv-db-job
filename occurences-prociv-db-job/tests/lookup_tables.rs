use occurences_prociv_db_job::lookup_tables::{
    Crepc, Csrepc, GroupedStatus, OccurrenceKind, OccurrenceKindFamily, OccurrenceKindGroup,
    OccurrenceStatus,
};
use setup::SetupUtils;

use sqlx::query_scalar;
use utils::lookup_table_test;

mod setup;

#[tokio::test]
pub async fn all_tables() {
    let setup = setup::setup().await;
    let SetupUtils { sqlx_pool, .. } = setup.utils();

    let lookup_tables: Vec<String> =
        query_scalar!(r#"SELECT "tablename" AS "tablename!" FROM pg_catalog.pg_tables WHERE "tablename" LIKE 'lu\_%';"#)
            .fetch_all(sqlx_pool)
            .await
            .unwrap();
    for table_name in lookup_tables {
        let table_name = table_name.as_str();
        match table_name {
            "lu_occurrence_kind_c_c" => {
                lookup_table_test::<OccurrenceKind, _>(table_name, sqlx_pool).await
            }
            "lu_occurrence_kind_c" => {
                lookup_table_test::<OccurrenceKindGroup, _>(table_name, sqlx_pool).await
            }
            "lu_crepc" => lookup_table_test::<Crepc, _>(table_name, sqlx_pool).await,
            "lu_csrepc" => lookup_table_test::<Csrepc, _>(table_name, sqlx_pool).await,
            "lu_grouped_status" => {
                lookup_table_test::<GroupedStatus, _>(table_name, sqlx_pool).await
            }
            "lu_occurrence_status" => {
                lookup_table_test::<OccurrenceStatus, _>(table_name, sqlx_pool).await
            }
            "lu_occurrence_kind" => {
                lookup_table_test::<OccurrenceKindFamily, _>(table_name, sqlx_pool).await
            }
            table_name => {
                panic!("lookup table not covered: {table_name}");
            }
        }
    }
}
mod utils {

    use database_utils::{LookupRecord, LookupTable};
    use sqlx::Executor;
    use std::str::FromStr;

    pub async fn lookup_table_test<'a, LU: LookupTable<'a, Database>, Database: sqlx::Database>(
        expected_table_name: &str,
        exec: impl Executor<'a, Database = Database> + Copy,
    ) where
        <LU as FromStr>::Err: std::fmt::Debug,
        <LU as TryFrom<i32>>::Error: std::fmt::Debug,
    {
        assert_eq!(LU::TABLE_NAME, expected_table_name);

        let res: Vec<LookupRecord> = LU::fetch_all_records().fetch_all(exec).await.unwrap();
        for LookupRecord { value, id } in res {
            let by_value = LU::from_str(&value).unwrap();
            let by_id = LU::try_from(id).unwrap();
            assert_eq!(by_value, by_id);
        }
        for variant in LU::iter() {
            let value: &str = variant.into();
            let id: i32 = variant.into();
            let by_value = LU::fetch_record_by_value(value)
                .fetch_one(exec)
                .await
                .unwrap();
            let by_value = LU::try_from(by_value.id).unwrap();

            let by_id = LU::fetch_record_by_id(id).fetch_one(exec).await.unwrap();
            let by_id = LU::try_from(by_id.id).unwrap();

            assert_eq!(variant, by_id);
            assert_eq!(variant, by_value);
            assert_eq!(by_value, by_id);
        }
    }
}
