use sqlx::PgPool;

use sqlx::migrate::Migrator;
use testcontainers::core::{ContainerPort, WaitFor};
use testcontainers::GenericImage;
use testcontainers_modules::testcontainers::ImageExt;
use testcontainers_modules::testcontainers::{runners::AsyncRunner, ContainerAsync};

static MIGRATOR: Migrator = sqlx::migrate!("../migrations");

pub struct Setup {
    utils: SetupUtils,
    _container_postgres: ContainerAsync<GenericImage>,
}

impl Setup {
    #[must_use]
    pub fn utils(&self) -> &SetupUtils {
        &self.utils
    }
}

pub struct SetupUtils {
    #[allow(dead_code)]
    pub sqlx_pool: PgPool,
}

pub async fn setup() -> Setup {
    let test_pg_image_tag = "17-3.5-alpine";
    let test_pg_image = "postgis/postgis";

    let container_postgres = GenericImage::new(test_pg_image, test_pg_image_tag)
        .with_exposed_port(ContainerPort::Tcp(5432))
        .with_wait_for(WaitFor::message_on_stdout(
            "database system is ready to accept connections",
        ))
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_DB", "postgres")
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .start()
        .await
        .unwrap();

    let connection_string = format!(
        "postgres://postgres:postgres@{}:{}/postgres",
        container_postgres.get_host().await.unwrap(),
        container_postgres.get_host_port_ipv4(5432).await.unwrap(),
    );

    dbg!(&connection_string);

    let sqlx_pool: sqlx::Pool<sqlx::Postgres> = PgPool::connect(&connection_string).await.unwrap();
    MIGRATOR.run(&sqlx_pool).await.unwrap();

    Setup {
        utils: SetupUtils { sqlx_pool },
        _container_postgres: container_postgres,
    }
}
