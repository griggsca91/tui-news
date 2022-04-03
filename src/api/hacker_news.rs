use std::{error::Error, fmt};

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct APIError {}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

pub fn api() -> Result<Vec<HNItem>, APIError> {
    let body = reqwest::blocking::get(
        "https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty",
    );

    if let Ok(result) = body {
        let parsed_body: _ = result.text().map_err(|_| APIError {})?;
        let new_response = format!("{{ \"ids\": {} }}", parsed_body);

        #[derive(Deserialize, Debug)]
        struct HNResult {
            ids: Vec<i32>,
        }

        let json: HNResult = serde_json::from_str(&*new_response).unwrap();
        let items: Vec<HNItem> = json
            .ids
            .iter()
            .take(10)
            .map(|id| get_hn_story(id).unwrap())
            .collect();
        return Ok(items);
    }

    Ok(vec![])
}

fn get_hn_story(id: &i32) -> Result<HNItem, Box<dyn Error>> {
    let body = reqwest::blocking::get(format!(
        "https://hacker-news.firebaseio.com/v0/item/{}.json",
        id
    ));

    match body {
        Ok(result) => {
            let response = result.text()?;
            let item: HNItem = serde_json::from_str(&*response)?;
            Ok(item)
        }
        Err(_) => Ok(HNItem {
            id: todo!(),
            title: todo!(),
            url: todo!(),
            score: todo!(),
            time: todo!(),
        }),
    }
}

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
