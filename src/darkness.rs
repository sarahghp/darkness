use anyhow::{Context, Result};
use reqwest::Error;
use serde::Deserialize;

use chrono::NaiveTime;
use chrono::Timelike;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    results: Times,
}

#[derive(Deserialize, Debug)]
struct Times {
    day_length: String,
}

pub async fn get_darkness(
    lat: f64,
    long: f64,
) -> Result<(std::string::String, u32), Box<dyn std::error::Error>> {
    let darkness_url = format!(
        "https://api.sunrise-sunset.org/json?lat={latitude}&lng={longitude}",
        latitude = lat,
        longitude = long,
    );

    let darkness_response = reqwest::get(&darkness_url).await?;
    let data: WeatherResponse = darkness_response.json().await?;
    let day_length = data.results.day_length;

    let parsed_day_length = NaiveTime::parse_from_str(&day_length, "%H:%M:%S")
        .with_context(|| "Unable to parse day length response. Sorry!")?;

    let hours = parsed_day_length.hour();
    let minutes = parsed_day_length.minute();

    let hours_of_darkness = match minutes {
        minutes if minutes > 30 => hours + 1,
        _ => hours,
    };

    Ok((day_length, hours_of_darkness))
}
