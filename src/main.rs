use structopt::StructOpt;
use exitfailure::{ExitFailure};

mod openweather;

// this struct holds the cli arguments
// they get read from stdin respectively,
#[derive(StructOpt)]
struct Opt {
    city: String,
    country_code: String,
    zip: String,
}

impl Opt {
    fn print(args: &Self) {
        println!("City: {}", args.city);
        println!("Country: {}", args.country_code);
        println!("Zip Code: {}", args.zip);
    }
}

// tokio allows our main fn to be async
#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Opt::from_args();
    // wait to get a Forecast response, instead of a Future
    let forecast = openweather::Forecast::get(&args.city, &args.country_code).await?;

    // Example: cargo run Chicago USA 60642 -> 
    // City: Chicago
    // Country: USA
    // High: 50°F
    // Low: 48°F
    // Humidity: 58%
    Opt::print(&args);
    println!("---------------");
    openweather::Forecast::print(&forecast);
    // this will round to the nearest tenths place

    Ok(())
}
