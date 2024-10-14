mod hot_top;
mod download;

use std::sync::Arc;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;
use crate::drivers::DownloadProviderManager;

pub struct AppState {
    pub download_provider: Arc<DownloadProviderManager>,
    pub pg: PgPool,
}