set dotenv-load

sqlx-migrate:
    cargo sqlx migrate run
sqlx-prepare: sqlx-migrate
    cargo sqlx prepare --workspace -- --all-targets --all-features
test:
    cargo test --all-features