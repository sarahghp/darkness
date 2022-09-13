#![allow(unused)]
use chrono::NaiveTime;
use chrono::Timelike;
use console::Style;
use std::io::{self, Write};

use reqwest::Error;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    results: Times,
}

#[derive(Deserialize, Debug)]
struct Times {
    day_length: String,
}

#[derive(Deserialize, Debug)]
struct Cities {
    name: String,
    country: String,
    latitude: f64,
    longitude: f64,
}

struct Dark {
    value: u32,
    as_string: String,
}

fn line_to_string(darkness: &Dark, val: i32, fill: &str) -> String {
    let pre_repeat = match darkness.value {
        d if d >= 10 => 7,
        _ => 8,
    };

    let amt_darkness = format!(
        "{}{}{}{}",
        " ".repeat(pre_repeat),
        darkness.as_string,
        " HOURS OF DARKNESS TODAY",
        " ".repeat(8)
    );

    let create_stars = |num: i32| fill.repeat(num.try_into().unwrap());

    match val {
        0 | 20 => String::from("*** ***"),
        1 | 19 => create_stars(15),
        2 | 18 => create_stars(23),
        3 | 17 => create_stars(29),
        4 | 16 => create_stars(33),
        5 | 15 => create_stars(37),
        6 | 14 => create_stars(39),
        7 | 13 => create_stars(41),
        8 | 9 | 11 | 12 => create_stars(43),
        10 => String::from(amt_darkness),
        _ => String::from(""),
    }
}

fn draw_output(darkness: Dark) {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    // let hours_of_darkness = 16;

    let percent_darkness = darkness.value as f64 / 24.;
    let lines_darkness = (20. * percent_darkness).round() as i32;

    let dark = Style::new().color256(93);
    let text = Style::new().color256(204);
    let light = Style::new().color256(220);

    for i in 0..21 {
        let main_string = match i {
            i if i < lines_darkness => line_to_string(&darkness, i, "O"),
            _ => line_to_string(&darkness, i, "o"),
        };

        let main_string_length = main_string.chars().count();
        let num_spaces = (51 - main_string_length) / 2;
        let spaces = " ".repeat(num_spaces);

        match i {
            20 => writeln!(handle, "{}{}", spaces, dark.apply_to(main_string)),
            0 => writeln!(handle, "{}{}", spaces, light.apply_to(main_string)),
            10 => writeln!(
                handle,
                "{}",
                format!("{}{}", spaces, text.apply_to(main_string),)
            ),
            x if x < lines_darkness => writeln!(
                handle,
                "{}",
                format!("{}{}", spaces, light.apply_to(main_string),)
            ),
            _ => writeln!(
                handle,
                "{}",
                format!("{}{}", spaces, dark.apply_to(main_string),)
            ),
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let city = "Melbourne";
    let country = "Australia";

    let lat_long_url = format!(
        "https://api.api-ninjas.com/v1/geocoding?city={city}&country={country}",
        city = city,
        country = country
    );

    let client = reqwest::Client::new();
    let lat_long_response = client
        .get(&lat_long_url)
        .header("X-Api-Key", "4n17BzkRjPu9HeA2mB/2mA==mtmV9crzbijIy6oH")
        .send()
        .await?;

    let cities: Vec<Cities> = lat_long_response.json().await?;

    let lat = cities[0].latitude;
    let long = cities[0].longitude;

    let request_url = format!(
        "https://api.sunrise-sunset.org/json?lat={latitude}&lng={longitude}",
        latitude = lat,
        longitude = long,
    );

    let response = reqwest::get(&request_url).await?;
    let data: WeatherResponse = response.json().await?;
    let day_length = data.results.day_length;

    println!("{}", day_length);

    let parsed_day_length = NaiveTime::parse_from_str(&day_length, "%H:%M:%S")?;

    let hours = parsed_day_length.hour();
    let minutes = parsed_day_length.minute();
    let hours_of_darkness = match minutes {
        minutes if minutes > 30 => hours + 1,
        _ => hours,
    };

    let dark = Dark {
        value: hours_of_darkness,
        as_string: day_length,
    };

    draw_output(dark);

    Ok(())
}
