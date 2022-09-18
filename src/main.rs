extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let city_url_prefix = "https://api.openweathermap.org/data/2.5/weather?appid=";
    let owm_api = &env::var("OPEN_WEATHER_MAP_API").unwrap() as &str;
    let query_key = "&q=";
    let city = "corner&20brook";

    let weather_url = city_url_prefix.to_owned() + owm_api + query_key + city;

    println!("{}", weather_url);
}

fn timestamp_to_datetime(epoch: i32) -> i32 {
    epoch
}

fn timestamp_to_time(epoch: i32) -> i32 {
    epoch
}

fn kelvin_to_celcius(kelvin_temp: f64) -> i32 {
    kelvin_temp as i32
}

fn deg_to_cardinal(deg: f64) -> f64 {
    let dirs = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", 
        "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW",
    ];
    deg
}

fn mps_to_kmph(mps: f64) -> i32 {
    mps as i32
}