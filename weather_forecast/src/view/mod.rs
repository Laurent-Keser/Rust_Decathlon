use crate::forecast;
use crate::forecast::WeatherData;
use dotenv::dotenv;
use reqwest::Client;
use std::env;
use std::io;

/**
 * Displays the selection menu of this project.
 */
pub async fn display_menu() {
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
                        0 => forecast::task1(&client, &api_key).await,
                        1 => forecast::task2(&client, &api_key).await,
                        2 => forecast::task2_new_city(&client, &api_key).await,
                        3 => forecast::task2_tomorrow(&client, &api_key).await,
                        _ => forecast::task1(&client, &api_key).await,
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
 * Prints the forecast based on the given weather data.
 */
pub fn print_forecast(forecast: &WeatherData) {
    println!("Weather in {} :", forecast.city);
    println!("\t- Time: {}", forecast.time);
    println!("\t- Weather: {}", forecast.weather);
    println!("\t- Description: {}", forecast.description);
    println!("\t- Temperature: {:.2} °C", forecast.temperature); // 2 decimals
    println!("\t- Humidity: {} %", forecast.humidity);
    println!("\t- Wind speed: {:.2} m/s", forecast.wind);
    println!();
}
