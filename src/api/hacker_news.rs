use std::{error::Error, fmt};

use crate::api::hnitem;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct APIError {}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

pub fn top_stories(limit: usize) -> Result<Vec<hnitem::HNItem>, APIError> {
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
        let items: Vec<hnitem::HNItem> = json
            .ids
            .iter()
            .take(limit)
            .map(|id| get_hn_story(id).unwrap())
            .collect();
        return Ok(items);
    }

    Ok(vec![])
}

fn get_hn_story(id: &i32) -> Result<hnitem::HNItem, Box<dyn Error>> {
    let body = reqwest::blocking::get(format!(
        "https://hacker-news.firebaseio.com/v0/item/{}.json",
        id
    ));

    match body {
        Ok(result) => {
            let response = result.text()?;
            let item: hnitem::HNItem = serde_json::from_str(&*response)?;
            Ok(item)
        }
        Err(_) => Ok(hnitem::HNItem::default()),
    }
}
