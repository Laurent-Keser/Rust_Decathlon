// Module import
mod forecast;

// External libraries (also specified in dependencies from Cargo.toml)
extern crate dotenv; // .env file (API_KEY)
extern crate reqwest; // HTTP calls
extern crate serde_json; // JSON parsing
extern crate tokio; // ASYNC blocks

// Imports of functions
use dotenv::dotenv;
use reqwest::Client;
use std::env;

/**
 * Prints the weather forecast of 10 Belgian cities
 */
#[tokio::main]
async fn main() {
    // Getting API_KEY from environment file
    dotenv().ok();

    let api_key: String = env::var("API_KEY").unwrap();
    let list_cities: Vec<&str> = forecast::get_cities();

    for city in &list_cities {
        let client = Client::new(); // HTTP client
        let forecast_result = forecast::get_weather_data(&client, &api_key, city)
            .await
            .unwrap();

        forecast::print_forecast(forecast_result);
    }
}
