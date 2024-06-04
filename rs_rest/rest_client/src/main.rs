extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct TickerData {
    ticker: String,
    //sentiment_rating: i32,
    // timestamp: i64,
    // positive: Option<String>,
    // neutral: Option<String>,
    // negative: Option<String>,
    // total: Option<String>,
    // next_earnings_date: Option<String>,
    // market_cap: Option<u64>,
    // options_oi_call_ratio: Option<String>,
    // #[serde(rename = "30_day_avg_iv")] // Rename to match JSON field
    // day_avg_iv: Option<f64>,
    // unusual_option_volume: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Make a GET request to a JSON API
    let response = reqwest::blocking::get("https://api.beta.swaggystocks.com/wsb/sentiment/rating?timeframe=12+hours")?;
    
    // Ensure the request was successful (status code 200)
    if !response.status().is_success() {
        println!("Failed to get response: {:?}", response);
        return Ok(());
    }
    
    // Read the response body as a string
    let body = response.text()?;
    
    // Deserialize JSON into vector of TickerData
    let ticker_data: Vec<TickerData> = serde_json::from_str(&body)?;

    // Extract ticker symbols and print
    for tix in ticker_data.iter().take(10) {
        println!("{}", tix.ticker);
    }
    
    Ok(())
}
