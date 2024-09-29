set dotenv-load

sqlx-prepare:
    cargo sqlx prepare --workspace -- --all-targets --all-features