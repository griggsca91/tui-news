use serde::Deserialize;

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug, Clone)]
pub struct HNItem {
    pub title: String,

    #[serde(default)]
    pub url: String,

    #[allow(dead_code)]
    #[serde(with = "ts_seconds")]
    time: DateTime<Utc>,
}

impl Default for HNItem {
    fn default() -> HNItem {
        HNItem {
            title: "".to_string(),
            url: "".to_string(),
            time: chrono::prelude::Utc::now(),
        }
    }
}
