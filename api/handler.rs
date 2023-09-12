use reqwest::blocking;
use scraper::{Html, Selector};
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Debug, Default)]
struct Contribution {
    count: u32,
    name: String,
    month: String,
    day: u32,
    year: u32,
    level: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let params = req.uri().path();
    print!("{:?}", params);
    let user = String::from("vinodnimbalkar");
    let year = 2022;
    let html = get_contributions(user, year);
    let _result = parse_contributions(html);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "你好，世界"
            })
            .to_string()
            .into(),
        )?)
    // .body(json!(result).to_string().into())?)
}

fn get_contributions(user: String, year: u32) -> String {
    let api = format!(
        "https://github.com/users/{}/contributions?from={}-12-01&to={}-12-31",
        user, year, year
    );
    let response = blocking::get(api).unwrap().text().unwrap();
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
