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
use tokio::runtime::Runtime;

/**
 * Prints the weather forecast of 10 Belgian cities
 */
fn main() {
    // Getting API_KEY from environment file
    dotenv().ok();

    let api_key: String = env::var("API_KEY").unwrap();
    let list_cities: Vec<&str> = forecast::get_cities();

    for city in &list_cities {
        let client = Client::new(); // HTTP client
        let _resp = client
            .get(&format!(
                "http://api.openweathermap.org/data/2.5/weather?q={},be&APPID={}&units=metric",
                city, api_key
            ))
            .send();
        // The main func can't be async so we need to have an async block of code because we need to wait for the response from the server
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let _resp = _resp.await;
            match _resp {
                Ok(forecast_result) => {
                    if forecast_result.status().is_success() {
                        println!("{}", forecast_result.status());
                        let result_json = forecast_result.json::<serde_json::Value>().await;

                        match result_json {
                            Ok(json) => {
                                // TODO: clean up
                                let weather = json["weather"][0]["main"].as_str().unwrap();
                                let description =
                                    json["weather"][0]["description"].as_str().unwrap();
                                let temperature = json["main"]["temp"].as_f64().unwrap();
                                let humidity = json["main"]["humidity"].as_i64().unwrap();
                                let wind = json["wind"]["speed"].as_f64().unwrap();

                                // Printing on screen
                                println!("Weather in {} :", city);
                                println!("\t- Weather: {}", weather);
                                println!("\t- Description: {}", description);
                                println!("\t- Temperature: {:.2} °C", temperature); // 2 decimals
                                println!("\t- Humidity: {} %", humidity);
                                println!("\t- Wind speed: {:.2} m/s", wind);
                                println!();
                            }
                            Err(err) => {
                                println!("Error in the parsing of the json response: {:?}", err);
                            }
                        }
                    } else {
                        println!("Error status of the request : {}", forecast_result.status());
                    }
                }

                Err(error) => println!("Error in the request: {:?}", error),
            }
        });
    }
}
