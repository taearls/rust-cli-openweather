use serde_derive::{Deserialize, Serialize};
use reqwest::Url;
use exitfailure::{ExitFailure};

// this struct holds the json response from the open weather api
// each struct represents a json object in the response payload
#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub coord: Coord,
    pub weather: Weather,
    pub base: String,
    pub main: Temps,
    pub visibility: i32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i32,
    pub sys: Sys,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temps {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    pub r#type: f64,
    pub id: i32,
    pub country: String,
    pub sunrise: i32,
    pub sunset: i32,
}

impl Forecast {
    // async because we're making an http request to open weather api endpoint
    pub async fn get(city: &str, country_code: &str) -> Result<Self, ExitFailure> {
        let openweather_api_key = dotenv::var("OPENWEATHER_API_KEY").unwrap();
        
        // api.openweathermap.org/data/2.5/weather?q={city name},{state code},{country code}&appid={API key}
        let url = format!("http://api.openweathermap.org/data/2.5/weather?q={},{}&units=imperial&appid={}", city, country_code, openweather_api_key);
        let url = Url::parse(&*url).expect(&format!("There was a problem parsing the url: {}", url));

        let response = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(response)
    }
}
