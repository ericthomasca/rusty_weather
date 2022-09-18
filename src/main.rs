use dotenv::dotenv;
use std::env;
use serde_derive::{Serialize, Deserialize};
use reqwest::Error;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherData {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    #[serde(rename = "temp_min")]
    pub temp_min: f64,
    #[serde(rename = "temp_max")]
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(rename = "sea_level")]
    pub sea_level: i64,
    #[serde(rename = "grnd_level")]
    pub grnd_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clouds {
    pub all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub id: i64,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}


fn main() {
    // Import .env file
    dotenv().ok();

    // Create weather api url
    let city_url_prefix = "https://api.openweathermap.org/data/2.5/weather?appid=";
    let owm_api = &env::var("OPEN_WEATHER_MAP_API").unwrap() as &str;
    let query_key = "&q=";
    let city = "corner&20brook";
    let weather_url = city_url_prefix.to_owned() + owm_api + query_key + city;

    println!("{}", weather_url); // to print the complete request url



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