mod database;
mod errors;
mod routes;

use crate::database::{database_connection, get_database_url};
use anyhow::Result;
use routes::get_routes;

#[tokio::main]
pub async fn start() -> Result<()> {
    let database_uri = get_database_url()?;
    let db = database_connection(&database_uri).await?;

    let app = get_routes(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
