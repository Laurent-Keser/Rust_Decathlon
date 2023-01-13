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
use std::io;

/**
 * Prints the weather forecast of 10 Belgian cities
 */
#[tokio::main]
async fn main() {
    // task1().await;
    task2().await;
}

async fn task1() {
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

async fn task2() {
    // Displaying list of cities
    println!("\n[List of cities]");
    let list_cities: Vec<&str> = forecast::get_cities();

    for (index, city) in list_cities.iter().enumerate() {
        println!("  {} - {}", index + 1, city);
    }

    // Reading user input and retrieving city
    let selected_city = read_city_input(list_cities);
    println!("You selected: {}\n", selected_city);

    // Displaying weather forecast
    dotenv().ok();
    let api_key: String = env::var("API_KEY").unwrap();
    let client = Client::new(); // HTTP client
    let forecast_result = forecast::get_weather_data(&client, &api_key, &selected_city)
        .await
        .unwrap();

    forecast::print_forecast(forecast_result);
}

/**
 * Reads the input from the user and loops until a valid city from the list is given.
 */
fn read_city_input(list_cities: Vec<&str>) -> String {
    let mut input = String::new();
    println!("\nSelect a city to display its weather forecast: ");

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let selected_index: Result<usize, _> = input.trim().parse();

        match selected_index {
            Ok(index) => {
                if index > 0 && index <= list_cities.len() {
                    return (&list_cities[index - 1]).to_string();
                } else {
                    println!(
                        "Please enter a valid number between 1 and {} : ",
                        list_cities.len()
                    );
                }
            }
            Err(_) => {
                println!(
                    "Please enter a valid number between 1 and {} : ",
                    list_cities.len()
                );
            }
        }
    }
}
