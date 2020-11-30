use structopt::StructOpt;
use exitfailure::{ExitFailure};
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

// this struct holds the cli arguments
// they get read from stdin respectively,
#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

// this struct holds the json response from the open weather api
// each struct represents a json object in the response payload
#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}

impl Forecast {
    // async because we're making an http request to open weather api endpoint
    async fn get(city: &str, country_code: &str) -> Result<Self, ExitFailure> {
        let openweather_api_key = dotenv::var("OPENWEATHER_API_KEY").unwrap();
        
        // api.openweathermap.org/data/2.5/weather?q={city name},{state code},{country code}&appid={API key}
        let url = format!("http://api.openweathermap.org/data/2.5/weather?q={},{}&units=imperial&appid={}", city, country_code, openweather_api_key);
        let url = Url::parse(&*url)?;

        let response = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(response)
    }
}

// tokio allows our main fn to be async
#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    // wait to get a Forecast response, instead of a Future
    let response = Forecast::get(&args.city, &args.country_code).await?;

    // Example: cargo run Chicago USA -> 
    // City: Chicago
    // Country: USA
    // High: 50°F
    // Low: 48°F
    // Humidity: 58%
    println!("City: {}", args.city);
    println!("Country: {}", args.country_code);
    println!("---------------");

    // this will round to the nearest tenths place
    println!("High: {:.1}°F", response.main.temp_max);
    println!("Low: {:.1}°F", response.main.temp_min);

    println!("Humidity: {}%", response.main.humidity);

    Ok(())
}
