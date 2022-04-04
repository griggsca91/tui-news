use serde::Deserialize;

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug, Clone)]
pub struct HNItem {
    id: i32,
    pub title: String,

    #[serde(default)]
    pub url: String,
    score: i32,

    #[serde(with = "ts_seconds")]
    time: DateTime<Utc>,
}

impl Default for HNItem {
    fn default() -> HNItem {
        HNItem {
            id: 0,
            title: "".to_string(),
            url: "".to_string(),
            score: 0,
            time: chrono::prelude::Utc::now(),
        }
    }
}
