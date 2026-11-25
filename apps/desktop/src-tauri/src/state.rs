use std::sync::Arc;

use devtime_application::ports::Clock;
use devtime_infrastructure::Database;

pub struct AppState {
    pub db: Arc<Database>,
    pub clock: Arc<dyn Clock>,
    pub started_at_ms: i64,
}
