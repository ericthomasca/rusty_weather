mod weather_data;

use dotenv::dotenv;
use std::env;
use exitfailure::ExitFailure;
use chrono::{NaiveDateTime, DateTime, Utc};

const KELVIN_ZERO: f64 = -273.15;
const MPS_TO_KMPH: f64 = 3.6;


#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    dotenv().ok();

    let api_key = &env::var("OPEN_WEATHER_MAP_API").unwrap() as &str;
    
    let args: Vec<String> = env::args().collect();
    let zip = &args[1].to_uppercase();
    let country = &args[2].to_uppercase();

    let data = weather_data::Root::get(&api_key, &zip, &country).await?;

    println!("=================================");
    println!("========  Rusty Weather  ========");
    println!("=================================");
    println!();

    let city = data.name;
    let lat = data.coord.lat;
    let lon = data.coord.lon;
    let timezone = data.timezone;

    let updated_timestamp = timestamp_to_datetime_str(data.dt, timezone);
    
    println!("Weather for {} ({}, {})", city, lat, lon);
    println!("Last Updated: {}", updated_timestamp);
    println!();
    
    let temperature = (data.main.temp + KELVIN_ZERO).round() as i32;
    let feels_like = (data.main.feels_like + KELVIN_ZERO).round() as i32;
    let conditions = &data.weather[0].main;

    println!("{}C (Feels like {}C) {}", temperature, feels_like, conditions);

    let temp_high = (data.main.temp_max + KELVIN_ZERO).round() as i32;
    let temp_low = (data.main.temp_min + KELVIN_ZERO). round() as i32;

    println!("High: {}C  Low: {}C", temp_high, temp_low);

    let wind_speed = (data.wind.speed * MPS_TO_KMPH).round();
    let wind_dir = deg_to_cardinal(data.wind.deg);

    println!("Wind: {}km/h {}", wind_speed, wind_dir);

    let sunrise_timestamp = timestamp_to_time_str(data.sys.sunrise, timezone);
    let sunset_timestamp = timestamp_to_time_str(data.sys.sunset, timezone);

    println!("Sunrise: {}  Sunset: {}", sunrise_timestamp, sunset_timestamp);    

    Ok(())

}


fn deg_to_cardinal(deg: i64) -> String {
    let dirs = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", 
        "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW",
    ];
    let ix = (((deg as f64) + 11.25) / 22.5).round() as i32;
    let dir_index = (ix % 16) as usize;
    let cardinal = dirs[dir_index];
    cardinal.to_owned()

}

fn timestamp_to_time_str(timestamp: i64, timezone: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(timestamp + timezone, 0);
    let datetime = DateTime::<Utc>::from_utc(naive, Utc);
    let timestamp_str = datetime.format("%H:%M:%S").to_string();
    timestamp_str
}

fn timestamp_to_datetime_str(timestamp: i64, timezone: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(timestamp + timezone, 0);
    let datetime = DateTime::<Utc>::from_utc(naive, Utc);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    timestamp_str
}