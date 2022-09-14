#![allow(unused)]
use anyhow::{Context, Result};

mod darkness;
use darkness::get_darkness;

mod draw;
use draw::{draw_output, Dark};

mod lat_lon;
use lat_lon::get_location;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let city = std::env::args()
        .nth(1)
        .with_context(|| "Please provide a city.")?;

    let country = std::env::args()
        .nth(2)
        .with_context(|| "Please provide a country.")?;

    let city_with_location = get_location(&city, &country).await?;

    let lat = city_with_location.latitude;
    let long = city_with_location.longitude;

    let (day_length, hours_of_darkness) = get_darkness(lat, long).await?;

    let dark = Dark {
        value: hours_of_darkness,
        as_string: day_length,
    };

    draw_output(dark);

    Ok(())
}
