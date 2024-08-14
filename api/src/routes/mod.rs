mod ingredient;

use axum::{
    routing::{delete, get, patch, post},
    Extension, Router,
};
use ingredient::{create_ingredient, delete_ingredient, get_ingredient, update_ingredient};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;
use tracing_appender::rolling;
use tracing_subscriber::{filter, prelude::*};

pub fn get_routes(database: DatabaseConnection) -> Router {
    // Add stdout and file logging
    let info_file = rolling::hourly("./logs", "info");
    let warning_file = rolling::hourly("./logs", "warning");
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    let info_layer = tracing_subscriber::fmt::layer()
        .with_writer(info_file)
        .with_ansi(false);
    let warning_layer = tracing_subscriber::fmt::layer()
        .with_writer(warning_file)
        .with_ansi(false);

    tracing_subscriber::registry()
        .with(stdout_log.with_filter(filter::LevelFilter::INFO))
        .with(info_layer.with_filter(filter::LevelFilter::INFO))
        .with(warning_layer.with_filter(filter::LevelFilter::WARN))
        .init();

    // Route handlers
    Router::new()
        .route("/ingredient/:id", get(get_ingredient))
        .route("/ingredient/create", post(create_ingredient))
        .route("/ingredient/update/:id", patch(update_ingredient))
        .route("/ingredient/:id", delete(delete_ingredient))
        .layer(Extension(database))
        .layer(TraceLayer::new_for_http())
}
