# Rusty Weather

A CLI weather app written is Rust.

## Usage

```bash
git clone https://github.com/ericthomasca/rusty_weather.git
cd rusty_weather
cargo build
cd target/debug
./rusty_weather {postal code} {country code}
```

## Example

```console
eric@term:~$ ./rusty_weather M5T CA
=================================
========  Rusty Weather  ========
=================================

Weather for Downtown Toronto (Kensington Market / Chinatown / Grange Park) (43.6541, -79.3978)
Last Updated: 2022-09-19 02:44:00

21C (Feels like 22C) Clear
High: 24C  Low: 19C
Wind: 11km/h SSW
Sunrise: 11:00:19  Sunset: 23:23:18
```