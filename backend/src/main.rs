use log::{debug, info, error};
use std::path::{Path, PathBuf};
use std::env;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer, HttpRequest, Result, middleware::Logger};
use actix_files::NamedFile;
use sqlx::postgres::PgPool;


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

// TODO:
// - create side-table (on startup?)
// 

async fn tables(pool: web::Data<PgPool>) -> Result<()> {
    let records = sqlx::query!(r#"
        SELECT *
        FROM pg_catalog.pg_tables
        WHERE schemaname != 'pg_catalog' AND 
        schemaname != 'information_schema';
    "#).fetch_all(pool.get_ref()).await?;

    for record in records {
        println!("{:?}", record);
    }

    Ok(())
}

/**
 * Entrypoint
 */
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "info,actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set.");

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

