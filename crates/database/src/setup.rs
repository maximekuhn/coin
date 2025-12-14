use std::str::FromStr;

pub async fn setup_database(db_file: &str) -> sqlx::Result<sqlx::SqlitePool> {
    let options = sqlx::sqlite::SqliteConnectOptions::from_str(db_file)?
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = sqlx::SqlitePool::connect_with(options).await?;
    apply_migrations(&pool).await?;
    Ok(pool)
}

async fn apply_migrations(pool: &sqlx::SqlitePool) -> sqlx::Result<()> {
    Ok(sqlx::migrate!().run(pool).await?)
}
