use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::{json, Value};
use std::net::SocketAddr;

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
        todo!()
    }
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `handler`
        .route("/:username/:year", get(handler));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn handler(Path((username, year)): Path<(String, u32)>) -> (StatusCode, Json<Value>) {
    let html = get_contributions(username, year).await;
    let result = parse_contributions(html);
    // Convert result to a JSON value
    let json_result = json!(result);
    (StatusCode::OK, Json(json_result))
}

async fn get_contributions(user: String, year: u32) -> String {
    let api = format!(
        "https://github.com/users/{}/contributions?from={}-12-01&to={}-12-31",
        user, year, year
    );
    // Create an async client
    let client = Client::new();
    let response = client.get(&api).send().await.unwrap().text().await.unwrap();
    response
}

fn parse_contributions(html: String) -> Vec<Vec<Contribution>> {
    let document = Html::parse_document(&html);
    let rows_selector = Selector::parse("tbody > tr").unwrap();
    let days_selector = Selector::parse("td:not(.ContributionCalendar-label)").unwrap();

    let mut contributions = Vec::new();

    for row in document.select(&rows_selector) {
        let days = row.select(&days_selector);

        let mut current_row = Vec::new();

        for day in days {
            let data = day.text().collect::<Vec<_>>();
            let data = data.first();
            if let Some(data) = data {
                let parts = data.split(" ");
                let data: Vec<&str> = parts.collect();
                if data.len() > 1 {
                    let contribution = Contribution {
                        count: if data[0] == "No" {
                            0
                        } else {
                            data[0].parse().unwrap()
                        },
                        name: data[3].replace(',', ""),
                        month: data[4].to_string(),
                        day: data[5].replace(',', "").parse().unwrap(),
                        year: data[6].parse().unwrap(),
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