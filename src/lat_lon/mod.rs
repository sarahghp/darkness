use anyhow::{Context, Result};
use reqwest::Error;
use serde::Deserialize;

mod secrets;

#[derive(Deserialize, Debug, Clone)]
pub struct CityWithLocation {
    pub name: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
}

pub async fn get_location(
    city: &str,
    country: &str,
) -> Result<(CityWithLocation), Box<dyn std::error::Error>> {
    let lat_long_url = format!(
        "https://api.api-ninjas.com/v1/geocoding?city={city}&country={country}",
        city = city,
        country = country
    );

    let client = reqwest::Client::new();
    let lat_long_response = client
        .get(&lat_long_url)
        .header("X-Api-Key", secrets::KEY)
        .send()
        .await
        .with_context(|| "Failed to get city location")?;

    let cities: Vec<CityWithLocation> = lat_long_response
        .json()
        .await
        .with_context(|| "Failed to parse city location response")?;

    let city_with_location = cities.get(0).with_context(|| format!("Unable to find long/lat for {}, {}. If the name has a space, try wrapping it in quotes.", city, country))?;

    Ok(city_with_location.clone())
}
