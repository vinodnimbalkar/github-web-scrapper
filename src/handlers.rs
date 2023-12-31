use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{extract::Path, http::StatusCode, Json};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct Contribution {
    count: u32,
    name: String,
    month: String,
    day: u32,
    year: u32,
    level: u32,
}
impl Contribution {
    fn default() -> Contribution {
        Contribution {
            count: 0,
            name: "".to_string(),
            month: "".to_string(),
            day: 0,
            year: 0,
            level: 0,
        }
    }
}
// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn handler(Path((username, year)): Path<(String, u32)>) -> impl IntoResponse {
    let html = get_contributions(username, year).await.unwrap();
    let result = parse_contributions(&html);
    // set header as content type application/json
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Cache-Control", "max-age=86400".parse().unwrap());
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    // Convert result to a JSON value
    let json_result = json!(result);
    (StatusCode::OK, headers, Json(json_result))
}

async fn get_contributions(user: String, year: u32) -> Result<String, reqwest::Error> {
    let api = format!(
        "https://github.com/users/{}/contributions?from={}-12-01&to={}-12-31",
        user, year, year
    );
    // Create an async client
    let client = Client::new();
    let response = client.get(&api).send().await.unwrap().text().await.unwrap();
    Ok(response)
}

fn parse_contributions(html: &str) -> Vec<Vec<Contribution>> {
    let document = Html::parse_document(&html);
    let rows_selector = Selector::parse("tbody > tr").unwrap();
    let days_selector = Selector::parse("td:not(.ContributionCalendar-label)").unwrap();

    let mut contributions = Vec::new();

    for row in document.select(&rows_selector) {
        let days = row.select(&days_selector);

        let mut current_row = Vec::new();

        for day in days {
            // let data = day.text().collect::<Vec<_>>();
            // let data = data.first();
            if let Some(data) = day.text().next() {
                let parts: Vec<_> = data.split_whitespace().collect();
                if parts.len() > 1 {
                    let contribution = Contribution {
                        count: if parts[0] == "No" {
                            0
                        } else {
                            parts[0].parse().unwrap()
                        },
                        name: parts[3].replace(',', ""),
                        month: parts[4].to_string(),
                        day: parts[5].replace(',', "").parse().unwrap(),
                        year: parts[6].parse().unwrap(),
                        level: day.value().attr("data-level").unwrap().parse().unwrap(),
                    };
                    current_row.push(contribution);
                } else {
                    current_row.push(Contribution::default());
                }
            }
        }

        contributions.push(current_row);
    }
    contributions
}
