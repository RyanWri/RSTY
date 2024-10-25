use std::collections::HashSet;
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use tokio;
use url::Url;
use serde::Serialize;
use polars::prelude::*;

// A struct to represent the crawler.
struct Crawler {
    visited: HashSet<String>,
    client: Client,
}

impl Crawler {
    pub fn new() -> Self {
        Crawler {
            visited: HashSet::new(),
            client: Client::new(),
        }
    }

    pub async fn crawl(&mut self, url: &str) -> Result<DataFrame, Box<dyn Error>> {
        // Avoid revisiting already visited URLs.
        if self.visited.contains(url) {
            return Ok(DataFrame::default());
        }

        self.visited.insert(url.to_string());
        println!("Visiting: {}", url);

        // Fetching the HTML content.
        let response = match self.client.get(url).send().await {
            Ok(resp) => resp.text().await?,
            Err(_) => {
                println!("Failed to fetch URL: {}", url);
                return Ok(DataFrame::default());
            }
        };
        let document = Html::parse_document(&response);

        // Extracting specific match data (e.g., team names, scores, etc.).
        let match_selector = Selector::parse(".match").unwrap();
        let mut matches = vec![];

        for element in document.select(&match_selector) {
            let team1_selector = Selector::parse(".team1").unwrap();
            let team2_selector = Selector::parse(".team2").unwrap();
            let score_selector = Selector::parse(".score").unwrap();

            let team1 = element.select(&team1_selector).next().map(|e| e.text().collect::<Vec<_>>().join("")).unwrap_or_default();
            let team2 = element.select(&team2_selector).next().map(|e| e.text().collect::<Vec<_>>().join("")).unwrap_or_default();
            let score = element.select(&score_selector).next().map(|e| e.text().collect::<Vec<_>>().join("")).unwrap_or_default();

            matches.push((team1, team2, score));
        }

        // Convert the matches to a DataFrame.
        let df = DataFrame::new(
            vec![
                Series::new("Team 1", matches.iter().map(|m| m.0.as_str()).collect::<Vec<_>>()),
                Series::new("Team 2", matches.iter().map(|m| m.1.as_str()).collect::<Vec<_>>()),
                Series::new("Score", matches.iter().map(|m| m.2.as_str()).collect::<Vec<_>>()),
            ]
        )?;

        Ok(df)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut crawler = Crawler::new();
    let df = crawler.crawl("https://www.soccerstats.com/matches.asp?matchday=0&daym=yesterday").await?;
    println!("{:#?}", df);
    Ok(())
}
