use std::time::Duration;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(60))
        .connect("postgres://user:password@postgres/postgres")
        .await?;

    sqlx::migrate!("db/migrations");

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    println!("{:?}", row);

    Ok(())
}
