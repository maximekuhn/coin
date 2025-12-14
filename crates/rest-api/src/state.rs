use crate::config;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: database::SqlitePool,
    pub config: config::Config,
}
