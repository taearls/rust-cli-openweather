# Rust CLI Weather App

This is a simple command line app built with Rust that returns information about the weather using the Open Weather API. 

## Setup

First, in order to use this application, you will need to create a free account at openweather and get a valid API key.

Then, after cloning the repository, you will need to create a `.env` file at the top level of the project with the following:

```
OPENWEATHER_API_KEY={your api key here}
```

After that, you can try running `cargo run Chicago USA 60647` to see what the weather is like in Logan Square in Chicago.

Example output:

```
cargo run Chicago USA 60647

City: Chicago
Country: USA
Zip Code: 60647
---------------
Temperature: 31.9°F
Wind: 3 miles/hr
Humidity: 61%
Description: Rain
```

## Future Improvements

I want to make this API as flexible as possible. With that in mind, I would like to make the command line arguments optional where it makes sense. For example, it should be valid to search only by zipcode.

Another future improvement is to add more robust usage descriptions and documentation that's accessible in the terminal.

