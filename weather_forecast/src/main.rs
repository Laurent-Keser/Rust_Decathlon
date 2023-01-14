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
    display_menu().await;
}

/**
 * Displays the selection menu of this project.
 */
async fn display_menu() {
    // Setup API key and HTTP client
    dotenv().ok();
    let api_key: String = env::var("API_KEY").unwrap();
    let client = Client::new(); // HTTP client

    // Displaying message
    println!("\n[*** RUST ASSESSMENT - DECATHLON ***]");
    println!("\nSelect an option: ");

    let options = vec![
        "Display the daily weather forecast of 10 Belgian cities",
        "Display the daily weather forecast of a city from the list",
        "Display the daily weather forecast from another city",
        "Display the weather forecast for tomorrow and the day after tomorrow from another city",
    ];

    for (index, option) in options.iter().enumerate() {
        println!("  {} - {}", index + 1, option);
    }

    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        println!("");
        let selected_index: Result<usize, _> = input.trim().parse();

        match selected_index {
            Ok(index) => {
                if index > 0 && index <= options.len() {
                    match index - 1 {
                        0 => task1(&client, &api_key).await,
                        1 => task2(&client, &api_key).await,
                        2 => task2_new_city(&client, &api_key).await,
                        3 => task2_tomorrow(&client, &api_key).await,
                        _ => task1(&client, &api_key).await,
                    }
                    break;
                } else {
                    println!(
                        "Please enter a valid number between 1 and {} : ",
                        options.len()
                    );
                }
            }
            Err(_) => {
                println!(
                    "Please enter a valid number between 1 and {} : ",
                    options.len()
                );
            }
        }
    }
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
        println!("");
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

/**
 * Displays the weather forecast for all 10 Belgian cities.
 */
async fn task1(client: &Client, api_key: &str) {
    let list_cities: Vec<&str> = forecast::get_cities();

    for city in &list_cities {
        let forecast_result = forecast::get_weather_data(&client, &api_key, city)
            .await
            .unwrap();
        forecast::print_forecast(&forecast_result);
    }
}

/**
 * Displays a list of city to choose from, to then display its weather forecast
 */
async fn task2(client: &Client, api_key: &str) {
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

    let forecast_result = forecast::get_weather_data(&client, &api_key, &selected_city)
        .await
        .unwrap();

    forecast::print_forecast(&forecast_result);
}
async fn task2_new_city(client: &Client, api_key: &str) {
    let mut input = String::new();

    loop {
        println!("\nWrite down a city to display its weather forecast: ");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        println!("");
        let selected_city = input.trim();
        let forecast_result =
            forecast::get_weather_data_unknown(&client, &api_key, &selected_city).await;

        match forecast_result {
            Ok(forecast) => {
                forecast::print_forecast(&forecast);
                break;
            }
            Err(err) => {
                println!("{}", err);
                println!("The city doesn't exist !");
            }
        }
    }
}

async fn task2_tomorrow(client: &Client, api_key: &str) {
    let mut input = String::new();

    loop {
        println!("\nWrite down a city to display its weather forecast: ");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        println!("");
        let selected_city = input.trim();
        let forecast_result =
            forecast::get_weather_data_tomorrow(&client, &api_key, &selected_city).await;

        match forecast_result {
            Ok(forecast) => {
                // Print each forecast
                println!("[TODAY]");
                forecast::print_forecast(&forecast[0]);

                println!("[TOMORROW]");
                forecast::print_forecast(&forecast[1]);

                println!("[AFTER TOMORROW]");
                forecast::print_forecast(&forecast[2]);

                break;
            }
            Err(err) => {
                println!("{}", err);
                println!("The city doesn't exist !");
            }
        }
    }
}
