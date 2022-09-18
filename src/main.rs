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
