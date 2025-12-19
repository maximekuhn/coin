use std::str::FromStr;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower::ServiceBuilder;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::AppState;

mod auth;
mod config;
mod error;
mod extractors;
mod handlers;
mod middlewares;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!(
            "usage: ./{} path/to/config/file",
            args.next().expect("program name is here")
        );
        std::process::exit(1);
    }

    let config_file_data = std::fs::read_to_string(args.nth(1).expect("args.len == 2"))?;
    let config: config::Config = config_file_data.parse()?;

    setup_logger(&config.log);

    let db_pool = database::setup::setup_database(&config.db_file).await?;
    let app_state = AppState { db_pool, config };

    let router = routes(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8757").await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}

fn routes(state: AppState) -> Router {
    let router = Router::new()
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/logout", post(handlers::auth::logout))
        .route("/groups", post(handlers::group::create))
        .route(
            "/groups/{group_id}/members",
            post(handlers::group::add_member),
        )
        .route("/hello", get(handlers::dummy::hello_user))
        .with_state(state);

    Router::new().nest("/api", router).layer(
        ServiceBuilder::new()
            .layer(middlewares::request_id::set_request_id_layer())
            .layer(middlewares::request_id::propagate_request_id_layer())
            .layer(middlewares::trace::trace_layer())
            .layer(middleware::from_fn(
                middlewares::sleep_unauthorized::sleep_on_401,
            )),
    )
}

fn setup_logger(lc: &config::LogConfig) {
    /// Crates for which logs will be displayed using the provided log level.
    const CRATES: [&str; 6] = [
        // workspace crates
        "application",
        "auth_models",
        "database",
        "domain",
        "rest_api",
        // external dependencies
        "tower_http",
    ];

    let level = tracing::Level::from_str(&lc.level).unwrap_or(tracing::Level::INFO);
    let mut filter = tracing_subscriber::filter::Targets::new();
    for c in CRATES {
        filter = filter.with_target(c, level);
    }
    filter = filter.with_target("sqlx", tracing_subscriber::filter::LevelFilter::OFF);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_current_span(false),
        )
        .with(filter)
        .init();
}
