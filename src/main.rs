use reqwest::blocking;
use scraper::{Html, Selector};
use std::fmt::Display;

#[derive(Debug, Default)]
struct Contribution {
    count: u32,
    name: String,
    month: String,
    day: u32,
    year: u32,
    level: u32,
}

fn main() {
    let api = "https://github.com/users/vinodnimbalkar/contributions?from=2022-12-01&to=2022-12-31";
    let response = blocking::get(api).unwrap().text().unwrap();
    let document = Html::parse_document(&response);
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
    print!("{:#?}", contributions);
}
