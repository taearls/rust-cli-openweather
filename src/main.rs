use structopt::StructOpt;
use exitfailure::{ExitFailure};

mod openweather;

// this struct holds the cli arguments
// they get read from stdin respectively,
#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

// tokio allows our main fn to be async
#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    // wait to get a Forecast response, instead of a Future
    let forecast = openweather::Forecast::get(&args.city, &args.country_code).await?;

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
    println!("High: {:.1}°F", forecast.main.temp_max);
    println!("Low: {:.1}°F", forecast.main.temp_min);

    println!("Humidity: {}%", forecast.main.humidity);

    Ok(())
}
