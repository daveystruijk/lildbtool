use log::{info};
use std::path::{Path, PathBuf};
use std::env;
use dotenv::dotenv;
use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, Result, middleware::Logger};
use actix_files::NamedFile;
use sqlx::{FromRow};
use sqlx::postgres::{PgPool};
use serde::{Serialize, Deserialize};


/**
 * Serve static files (at ../frontend/build/)
 */
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("../frontend/build/index.html"))?)
}

async fn files(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(Path::new("../frontend/build").join(path))?)
}

/**
 * API
 */

#[derive(FromRow, Serialize, Deserialize)]
struct PgTable {
    schemaname: String,
    tablename: String
}

async fn tables(pool: web::Data<PgPool>) -> HttpResponse {
    let pg_tables = sqlx::query_as::<_, PgTable>(r#"
        SELECT *
        FROM pg_catalog.pg_tables
        WHERE schemaname != 'pg_catalog'
        AND schemaname != 'information_schema'
        AND schemaname != 'lildbtool';
    "#).fetch_all(pool.get_ref()).await.unwrap();

    let table_names: Vec<String> = pg_tables.into_iter()
        .map(|pg_table| pg_table.tablename).collect();

    let json = serde_json::to_string(&table_names).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}

/**
 * Entrypoint
 */
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "info,actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set.");

    info!("Connecting to {}", database_url);
    let pool = PgPool::connect(&database_url).await?;

    let listen_addr = "127.0.0.1:8080";
    HttpServer::new(move || App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .route("/api/tables", web::get().to(tables))
            .route("/", web::get().to(index))
            .route("/{filename:.*}", web::get().to(files))
        ).bind(listen_addr)?
        .run()
        .await?;

    Ok(())
}

