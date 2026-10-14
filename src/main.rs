use sea_orm::Database;

pub mod db;
pub mod entities;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let db_url = std::env::var("DATABASE_URL")?;
    let db = Database::connect(db_url).await?;

    db.get_schema_registry(module_path!().split("::").next().unwrap())
        .sync(&db)
        .await?;

    // TODO: close connection on graceful shutdown

    Ok(())
}
