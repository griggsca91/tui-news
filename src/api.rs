use chrono::Utc;
use serde::Deserialize;

pub fn api() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get(
        "https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty",
    );

    if let Ok(result) = body {
        let parsed_body: _ = result.text()?;
        let new_response = format!("{{ \"ids\": {} }}", parsed_body);

        #[derive(Deserialize, Debug)]
        struct HNResult {
            ids: Vec<i32>,
        }

        let json: HNResult = serde_json::from_str(&*new_response).unwrap();
        println!("{:?}", json);
        println!("{:?}", json.ids);
    }

    println!("junk");

    Ok(())
}

pub struct HNItem {
    id: String,
    title: String,
    url: String,
    score: i32,
    time: chrono::DateTime<Utc>,
}
