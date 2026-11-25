use serde_json::Value;

use crate::errors::ApplicationError;
use crate::ports::Preferences;

pub const KEY_MAX_LEN: usize = 64;

fn validate_key(key: &str) -> Result<(), ApplicationError> {
    let valid = !key.is_empty()
        && key.len() <= KEY_MAX_LEN
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
    if valid {
        Ok(())
    } else {
        Err(ApplicationError::InvalidKey(key.to_string()))
    }
}

pub fn get_preference<P: Preferences>(
    repo: &P,
    key: &str,
) -> Result<Option<String>, ApplicationError> {
    validate_key(key)?;
    repo.get(key).map_err(ApplicationError::from)
}

pub fn set_preference<P: Preferences>(
    repo: &P,
    key: &str,
    value: &Value,
) -> Result<(), ApplicationError> {
    validate_key(key)?;
    let raw = serde_json::to_string(value)?;
    repo.set(key, &raw).map_err(ApplicationError::from)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Mutex;

    use super::*;
    use crate::errors::PreferenceError;

    #[derive(Default)]
    struct FakePreferences {
        values: Mutex<HashMap<String, String>>,
    }

    impl Preferences for FakePreferences {
        fn get(&self, key: &str) -> Result<Option<String>, PreferenceError> {
            let values = self.values.lock().unwrap();
            Ok(values.get(key).cloned())
        }

        fn set(&self, key: &str, value_json: &str) -> Result<(), PreferenceError> {
            let mut values = self.values.lock().unwrap();
            values.insert(key.to_string(), value_json.to_string());
            Ok(())
        }
    }

    #[test]
    fn stores_and_reads_json_preferences() {
        let repo = FakePreferences::default();
        set_preference(&repo, "appearance.theme", &serde_json::json!("dark")).unwrap();
        assert_eq!(
            get_preference(&repo, "appearance.theme").unwrap(),
            Some("\"dark\"".to_string())
        );
    }

    #[test]
    fn rejects_unsafe_keys() {
        let repo = FakePreferences::default();
        assert!(get_preference(&repo, "appearance theme").is_err());
        assert!(get_preference(&repo, "").is_err());
        assert!(get_preference(&repo, &"a".repeat(KEY_MAX_LEN + 1)).is_err());
        assert!(set_preference(&repo, "drop;table", &serde_json::json!(1)).is_err());
    }

    #[test]
    fn accepts_known_key_shapes() {
        let repo = FakePreferences::default();
        for key in [
            "appearance.theme",
            "sync.last_sync_at",
            "privacy.telemetry-opt-in",
        ] {
            assert!(get_preference(&repo, key).is_ok(), "{key}");
        }
    }
}
