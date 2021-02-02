use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, time::Instant};

#[tokio::main()]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .expect("failed to create pool");

    let start = Instant::now();
    let _ = sqlx::query("SELECT text from tweet WHERE owner_id = $1")
        .bind(2_i32)
        // NOTE: Using `persistent(false)` to avoid prepared query cache.
        .persistent(false)
        .fetch_all(&pool)
        .await;
    println!("query with i32 takes {:?}", start.elapsed());

    let start = Instant::now();
    let _ = sqlx::query("SELECT text from tweet WHERE owner_id = $1")
        .bind(2_u32)
        .persistent(false)
        .fetch_all(&pool)
        .await;
    println!("query with u32 takes {:?}", start.elapsed());
}
