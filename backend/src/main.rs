use std::env;

use axum::{http::StatusCode, routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    query_as,
};
use ts_rs::TS;

#[derive(TS, Serialize)]
#[ts(export)]
struct TablesResponse {
    id: u64,
    username: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or("0.0.0.0:4001".to_string());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    tracing_subscriber::fmt::init();

    let pg = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/tables", get(get_tables))
        .layer(Extension(pg));

    axum::Server::bind(&listen_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
struct Table {
    table_catalog: Option<String>,
    table_schema: Option<String>,
    table_name: Option<String>,
    table_type: Option<String>,
    self_referencing_column_name: Option<String>,
    reference_generation: Option<String>,
    user_defined_type_catalog: Option<String>,
    user_defined_type_schema: Option<String>,
    user_defined_type_name: Option<String>,
    is_insertable_into: Option<String>,
    is_typed: Option<String>,
    commit_action: Option<String>,
}

async fn get_tables(
    Extension(pg): Extension<PgPool>,
) -> Result<Json<Vec<Table>>, (StatusCode, &'static str)> {
    let tables = query_as!(
        Table,
        r#"
            SELECT * FROM information_schema.tables
            WHERE table_schema != 'information_schema'
            AND table_schema != 'pg_catalog'
        "#
    )
    .fetch_all(&pg)
    .await
    .unwrap();

    Ok(Json(tables))
}
