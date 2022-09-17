#![allow(unused)]
use anyhow::{Context, Result, bail};
use console::Style;


use config::Config;
use std::collections::HashMap;

mod darkness;
use darkness::get_darkness;

mod draw;
use draw::{draw_output, Dark};

mod lat_lon;
use lat_lon::get_location;

#[derive(Debug)]
struct Settings {
    key: Option<String>,
    points:  Option<String>,
    arg1:  Option<String>,
    arg2:  Option<String>,
}

async fn draw_with_location(lat: f64, long: f64) -> Result<(), Box<dyn std::error::Error>> {

    let (dark_length, hours_of_darkness) = get_darkness(lat, long).await?;

    let dark = Dark {
        value: hours_of_darkness,
        as_string: dark_length,
    };

    draw_output(dark);

    Ok(())
}

// fn location_str_to_f64(points: &str) -> Result<(f64, f64), Box<dyn std::error::Error>> {
//     let (lat_str, long_str) = points.split_once(" ").unwrap();
//     let lat = lat_str.parse::<f64>()?;
//     let long = long_str.parse::<f64>()?;
//     println!("{}{}", lat, long);
//     Ok((lat, long))
// }

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {


    /*
     * There are two ways to provide the information the
     * application needs to run
     *
     * (1) Provide API key in config or env, plus city/country in std::env
     * (2) Provide lat/long in config or as flagged arg
     *
     *  */

     let config = Config::builder()
        .add_source(config::File::with_name("config").required(false))
        .add_source(config::Environment::with_prefix("DARK"))
        .build()
        .unwrap_or_default();

    //  println!(
    //     "{:?}",
    //     settings
    //         .try_deserialize::<HashMap<String, String>>()
    //         .unwrap()
    // );
    //

    let settings = Settings {
        points: config.get::<String>("points").ok(),
        key: config.get::<String>("key").ok(),
        arg1: std::env::args().nth(1),
        arg2: std::env::args().nth(2),
    };

    println!("{:?}", settings);
    let e_pink = Style::new().color256(197);


    match settings {
        Settings { points: None, key: None, arg1: None, arg2: None } => {
            bail!(e_pink.apply_to("\nTo use darkness-check please provide a location in the form:\ndarkness-check <LAT> <LONG> \nor see documentation for other options."))
        },
        Settings { points: None, key: Some(_), arg1: None, arg2: None } => {
            bail!(e_pink.apply_to("\nYou have provided a key but no location\nPlease provide city and country in the form:\ndarkness-check -- berlin germany\nor see documentation for other options."))
        },
        Settings { key: None, arg1: Some(lat_str), arg2: Some(long_str), .. } => {
            let lat = lat_str.parse::<f64>()?;
            let long = long_str.parse::<f64>()?;
            draw_with_location(lat, long).await;
        },
        Settings { points: Some(points), arg1: None, .. } => {
            let (lat_str, long_str) = points.split_once(" ").unwrap();
            let lat = lat_str.parse::<f64>()?;
            let long = long_str.parse::<f64>()?;
            draw_with_location(lat, long).await;
        },
        Settings { key: Some(key), arg1: Some(city), arg2: Some(country), .. } => {
            let city_with_location = get_location(&city, &country).await?;
            let lat = city_with_location.latitude;
            let long = city_with_location.longitude;
            draw_with_location(lat, long).await;
        },
        _ => todo!(), // error, this is a surprise
    }



    // let city = std::env::args()
    //     .nth(1)
    //     .with_context(|| "Please provide a city.")?;

    // let country = std::env::args()
    //     .nth(2)
    //     .with_context(|| "Please provide a country.")?;

    // let city_with_location = get_location(&city, &country).await?;

    // let lat = city_with_location.latitude;
    // let long = city_with_location.longitude;

    // let (dark_length, hours_of_darkness) = get_darkness(lat, long).await?;

    // let dark = Dark {
    //     value: hours_of_darkness,
    //     as_string: dark_length,
    // };

    // draw_output(dark);
    //


    Ok(())
}
