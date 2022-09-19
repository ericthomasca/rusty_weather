mod weather_data;

use dotenv::dotenv;
use std::env;
use exitfailure::ExitFailure;
use chrono::{NaiveDate, NaiveDateTime, DateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

const KELVIN_ZERO: f64 = -273.15;


#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    dotenv().ok();

    let api_key = &env::var("OPEN_WEATHER_MAP_API").unwrap() as &str;
    let city = "toronto";
    let res = weather_data::Root::get(&api_key, &city).await?;

    println!("=====================");
    println!("==  Rusty Weather  ==");
    println!("=====================");
    println!();

    let city = res.name;
    let lat = res.coord.lat;
    let lon = res.coord.lon;

    let updated_timestamp = res.dt;
    let updated_naive = NaiveDateTime::from_timestamp(updated_timestamp, 0);
    let updated_datetime = DateTime::<Utc>::from_utc(updated_naive, Utc);
    let updated_timestamp_str = updated_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    
    println!("Weather for {} ({}, {})", city, lat, lon);
    println!("Last Updated: {}", updated_timestamp_str);
    println!();
    
    let temperature = (res.main.temp + KELVIN_ZERO).round() as i32;
    let feels_like = (res.main.feels_like + KELVIN_ZERO).round() as i32;
    let conditons = &res.weather[0].main;

    println!("{}C (Feels like {}C) {}", temperature, feels_like, conditons);

    let temp_high = (res.main.temp_max + KELVIN_ZERO).round() as i32;
    let temp_low = (res.main.temp_min + KELVIN_ZERO). round() as i32;

    println!("High: {}C  Low: {}C", temp_high, temp_low);

    let wind_speed = (res.wind.speed * 3.6).round();
    let wind_dir = deg_to_cardinal(res.wind.deg);

    println!("Wind: {}km/h {}", wind_speed, wind_dir);

    let sunrise_timestamp = res.sys.sunrise;
    let sunrise_naive = NaiveDateTime::from_timestamp(sunrise_timestamp, 0);
    let sunrise_datetime = DateTime::<Utc>::from_utc(sunrise_naive, Utc);
    let sunrise_timestamp_str = sunrise_datetime.format("%H:%M:%S").to_string();

    let sunset_timestamp = res.sys.sunset;
    let sunset_naive = NaiveDateTime::from_timestamp(sunset_timestamp, 0);
    let sunset_datetime = DateTime::<Utc>::from_utc(sunset_naive, Utc);
    let sunset_timestamp_str = sunset_datetime.format("%H:%M:%S").to_string();

    println!("Sunrise: {}  Sunset: {}", sunrise_timestamp_str, sunset_timestamp_str);    

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
