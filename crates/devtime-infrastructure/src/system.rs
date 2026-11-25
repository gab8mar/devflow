use std::time::{SystemTime, UNIX_EPOCH};

use devtime_application::ports::Clock;

#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_ms(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as i64)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_current_epoch_millis() {
        let now = SystemClock.now_ms();
        assert!(now > 1_600_000_000_000);
    }
}
