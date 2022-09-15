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
    println!(" 💌 Fetching darkness ...");

    let darkness_url = format!(
        "https://api.sunrise-sunset.org/json?lat={latitude}&lng={longitude}",
        latitude = lat,
        longitude = long,
    );

    let darkness_response = reqwest::get(&darkness_url).await?;
    let data: WeatherResponse = darkness_response.json().await?;
    let day_length = data.results.day_length;

    println!(" 🌑 Considering darkness ...\n\n");

    let parsed_day_length = NaiveTime::parse_from_str(&day_length, "%H:%M:%S")
        .with_context(|| "Unable to parse day length response. Sorry!")?;

    let inverter = NaiveTime::from_hms(23, 59, 59);
    let parsed_darkness_duration = inverter - parsed_day_length;
    let parsed_darkness = NaiveTime::from_num_seconds_from_midnight(
        parsed_darkness_duration.num_seconds().try_into().unwrap(),
        0,
    );

    let hours = parsed_darkness.hour();
    let minutes = parsed_darkness.minute();

    let hours_of_darkness = match minutes {
        minutes if minutes > 30 => hours + 1,
        _ => hours,
    };

    Ok((parsed_darkness.to_string(), hours_of_darkness))
}
