use crate::model;
use dotenv::dotenv;
use reqwest::Client;
use std::env;
use std::io;

/**
 * Launches the program and displays the selection menu.
 */
pub async fn start() {
    // Setup API key and HTTP client
    dotenv().ok();
    let api_key: String = env::var("API_KEY").unwrap();
    let client = Client::new(); // HTTP client

    // Displaying menu
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
                        0 => model::all_cities_weather(&client, &api_key).await,
                        1 => model::selected_city_weather(&client, &api_key).await,
                        2 => model::new_city_weather(&client, &api_key).await,
                        3 => model::next_days_weather(&client, &api_key).await,
                        _ => model::all_cities_weather(&client, &api_key).await,
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
