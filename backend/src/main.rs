use std::env;

use axum::{extract::Path, http::StatusCode, routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    query, query_as,
};
use ts_rs::TS;

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
        .route("/api/tables", get(get_tables))
        .route("/api/tables/:id", get(get_table_details))
        .layer(Extension(pg));

    axum::Server::bind(&listen_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
struct TableResult {
    table_name: Option<String>,
}

async fn get_tables(
    Extension(pg): Extension<PgPool>,
) -> Result<Json<Vec<TableResult>>, (StatusCode, &'static str)> {
    let tables = query_as!(
        TableResult,
        r#"
            SELECT table_name
            FROM information_schema.tables
            WHERE table_schema = 'public'
        "#
    )
    .fetch_all(&pg)
    .await
    .unwrap();

    // println!("{:?}", tables);

    Ok(Json(tables))
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
struct TableDetailResult {
    column_name: Option<String>,
    data_type: Option<String>,
}

async fn get_table_details(
    Extension(pg): Extension<PgPool>,
    Path(table_name): Path<String>,
) -> Result<Json<Vec<TableDetailResult>>, (StatusCode, &'static str)> {
    let details = query_as!(
        TableDetailResult,
        r#"
            SELECT column_name, data_type
            FROM information_schema.columns
            WHERE table_schema = 'public' AND 
            table_name = $1
        "#,
        table_name,
    )
    .fetch_all(&pg)
    .await
    .unwrap();

    Ok(Json(details))
}
