extern crate dotenv; // .env file (API_KEY)
use chrono::prelude::*;
use dotenv::dotenv;
use reqwest::Client;
use std::error::Error;
use std::io;

use crate::view::print_forecast;

pub struct WeatherData {
    pub city: String,
    pub weather: String,
    pub description: String,
    pub temperature: f64,
    pub humidity: i64,
    pub wind: f64,
    pub time: String,
}

/**
 * Returns a list of my 10 favorite Belgian cities.
 */
pub fn get_cities() -> Vec<&'static str> {
    return vec![
        "Brussels",
        "Mons",
        "Liège",
        "Eupen",
        "Namur",
        "Braine-le-Comte",
        "Soignies",
        "Charleroi",
        "Dinant",
        "Nivelles",
    ];
}

/**
 * Gets the weather forecast of the given city.
 */
pub async fn get_weather_data(
    client: &Client,
    api_key: &str,
    city: &str,
) -> Result<WeatherData, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    let response = client.get(&url).send().await;

    match response {
        Ok(api_response) => {
            if api_response.status().is_success() {
                //  println!("{}", api_response.status());

                let result = api_response.json::<serde_json::Value>().await;
                match result {
                    Ok(forecast) => {
                        let city = city.to_string();
                        let weather = forecast["weather"][0]["main"].as_str().unwrap().to_string();
                        let description = forecast["weather"][0]["description"]
                            .as_str()
                            .unwrap()
                            .to_string();
                        let temperature = forecast["main"]["temp"].as_f64().unwrap();
                        let humidity = forecast["main"]["humidity"].as_i64().unwrap();
                        let wind = forecast["wind"]["speed"].as_f64().unwrap();
                        let time = Local::now().to_string();

                        let weather_data = WeatherData {
                            city,
                            weather,
                            description,
                            temperature,
                            humidity,
                            wind,
                            time,
                        };

                        return Ok(weather_data);
                    }
                    Err(err) => {
                        println!("Error in the parsing of the json response: {:?}", err);
                        return Err(err);
                    }
                }
            } else {
                panic!("Error status of the request : {}", api_response.status());
            }
        }
        Err(err) => {
            println!("Error in the request: {:?}", err);
            return Err(err);
        }
    }
}

/**
 * Gets the weather forecast of the given city.
 */
pub async fn get_weather_data_unknown(
    client: &Client,
    api_key: &str,
    city: &str,
) -> Result<WeatherData, Box<dyn Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    let response = client.get(&url).send().await;

    match response {
        Ok(api_response) => {
            if api_response.status().is_success() {
                //  println!("{}", api_response.status());

                let result = api_response.json::<serde_json::Value>().await;
                match result {
                    Ok(forecast) => {
                        let city = city.to_string();
                        let weather = forecast["weather"][0]["main"].as_str().unwrap().to_string();
                        let description = forecast["weather"][0]["description"]
                            .as_str()
                            .unwrap()
                            .to_string();
                        let temperature = forecast["main"]["temp"].as_f64().unwrap();
                        let humidity = forecast["main"]["humidity"].as_i64().unwrap();
                        let wind = forecast["wind"]["speed"].as_f64().unwrap();
                        let time = Local::now().to_string();

                        let weather_data = WeatherData {
                            city,
                            weather,
                            description,
                            temperature,
                            humidity,
                            wind,
                            time,
                        };

                        return Ok(weather_data);
                    }
                    Err(err) => {
                        println!("Error in the parsing of the json response: {:?}", err);
                        return Err(err.into());
                    }
                }
            } else {
                let error = format!("Error status of the request : {}", api_response.status());
                return Err(error.into());
            }
        }
        Err(err) => {
            println!("Error in the request: {:?}", err);
            return Err(err.into());
        }
    }
}

pub async fn get_weather_data_tomorrow(
    client: &Client,
    api_key: &str,
    city: &str,
) -> Result<Vec<WeatherData>, Box<dyn Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units=metric&cnt=17",
        city, api_key
    );
    let response = client.get(&url).send().await;

    match response {
        Ok(api_response) => {
            if api_response.status().is_success() {
                //  println!("{}", api_response.status());

                let result = api_response.json::<serde_json::Value>().await;
                match result {
                    Ok(result) => {
                        // 0 - 8 - 16 to have 24 hours between each forecast
                        let mut count = 0;
                        let mut forecasts = Vec::new();

                        while count <= 16 {
                            let forecast = &result["list"][count];

                            let city = city.to_string();
                            let weather =
                                forecast["weather"][0]["main"].as_str().unwrap().to_string();
                            let description = forecast["weather"][0]["description"]
                                .as_str()
                                .unwrap()
                                .to_string();
                            let temperature = forecast["main"]["temp"].as_f64().unwrap();
                            let humidity = forecast["main"]["humidity"].as_i64().unwrap();
                            let wind = forecast["wind"]["speed"].as_f64().unwrap();
                            let time = forecast["dt_txt"].as_str().unwrap().to_string();

                            let weather_data = WeatherData {
                                city,
                                weather,
                                description,
                                temperature,
                                humidity,
                                wind,
                                time,
                            };

                            forecasts.push(weather_data);
                            count += 8;
                        }

                        return Ok(forecasts);
                    }
                    Err(err) => {
                        println!("Error in the parsing of the json response: {:?}", err);
                        return Err(err.into());
                    }
                }
            } else {
                let error = format!("Error status of the request : {}", api_response.status());
                return Err(error.into());
            }
        }
        Err(err) => {
            println!("Error in the request: {:?}", err);
            return Err(err.into());
        }
    }
}

/**
 * Reads the input from the user and loops until a valid city from the list is given.
 */
pub fn read_city_input(list_cities: Vec<&str>) -> String {
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
pub async fn task1(client: &Client, api_key: &str) {
    let list_cities: Vec<&str> = get_cities();

    for city in &list_cities {
        let forecast_result = get_weather_data(&client, &api_key, city).await.unwrap();
        print_forecast(&forecast_result);
    }
}

/**
 * Displays a list of city to choose from, to then display its weather forecast
 */
pub async fn task2(client: &Client, api_key: &str) {
    // Displaying list of cities
    println!("\n[List of cities]");
    let list_cities: Vec<&str> = get_cities();

    for (index, city) in list_cities.iter().enumerate() {
        println!("  {} - {}", index + 1, city);
    }

    // Reading user input and retrieving city
    let selected_city = read_city_input(list_cities);
    println!("You selected: {}\n", selected_city);

    // Displaying weather forecast
    dotenv().ok();

    let forecast_result = get_weather_data(&client, &api_key, &selected_city)
        .await
        .unwrap();

    print_forecast(&forecast_result);
}
pub async fn task2_new_city(client: &Client, api_key: &str) {
    let mut input = String::new();

    loop {
        println!("\nWrite down a city to display its weather forecast: ");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        println!("");
        let selected_city = input.trim();
        let forecast_result = get_weather_data_unknown(&client, &api_key, &selected_city).await;

        match forecast_result {
            Ok(forecast) => {
                print_forecast(&forecast);
                break;
            }
            Err(err) => {
                println!("{}", err);
                println!("The city doesn't exist !");
            }
        }
    }
}

pub async fn task2_tomorrow(client: &Client, api_key: &str) {
    let mut input = String::new();

    loop {
        println!("\nWrite down a city to display its weather forecast: ");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        println!("");
        let selected_city = input.trim();
        let forecast_result = get_weather_data_tomorrow(&client, &api_key, &selected_city).await;

        match forecast_result {
            Ok(forecast) => {
                // Print each forecast
                println!("[TODAY]");
                print_forecast(&forecast[0]);

                println!("[TOMORROW]");
                print_forecast(&forecast[1]);

                println!("[AFTER TOMORROW]");
                print_forecast(&forecast[2]);

                break;
            }
            Err(err) => {
                println!("{}", err);
                println!("The city doesn't exist !");
            }
        }
    }
}
