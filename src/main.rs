use structopt::StructOpt;

// this struct holds the cli arguments
// they get read from stdin respectively,
#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

fn main() {
    let args = Cli::from_args();
    // cargo run Chicago USA -> City is Chicago, Country Code is USA
    println!("City is {}, Country Code is {}", args.city, args.country_code);
}
