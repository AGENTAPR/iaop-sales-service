use std::sync::Arc;

use iaop_sales_service_infrastructure::config::AppConfig;
use sqlx::PgPool;

/// Shared application state injected into handlers via `State`.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    /// `None` when DATABASE_URL is not configured; `/health/ready`
    /// reports accordingly instead of failing.
    pub db: Option<PgPool>,
}
