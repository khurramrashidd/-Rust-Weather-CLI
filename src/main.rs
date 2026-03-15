use std::env;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct Weather {
    current_weather: CurrentWeather,
}

#[derive(Deserialize)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <city>");
        return;
    }

    let city = &args[1];

    let url = "https://api.open-meteo.com/v1/forecast?latitude=19.07&longitude=72.87&current_weather=true";

    let response = reqwest::blocking::get(url)
        .unwrap()
        .json::<Weather>()
        .unwrap();

    println!("City: {}", city);
    println!("Temperature: {}°C", response.current_weather.temperature);
    println!("Wind Speed: {} km/h", response.current_weather.windspeed);
}