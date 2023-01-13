use reqwest::Client;

pub struct WeatherData {
    pub city: String,
    pub weather: String,
    pub description: String,
    pub temperature: f64,
    pub humidity: i64,
    pub wind: f64,
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

                        let weather_data = WeatherData {
                            city,
                            weather,
                            description,
                            temperature,
                            humidity,
                            wind,
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
 * Prints the forecast based on the given weather data.
 */
pub fn print_forecast(forecast: WeatherData) {
    println!("Weather in {} :", forecast.city);
    println!("\t- Weather: {}", forecast.weather);
    println!("\t- Description: {}", forecast.description);
    println!("\t- Temperature: {:.2} °C", forecast.temperature); // 2 decimals
    println!("\t- Humidity: {} %", forecast.humidity);
    println!("\t- Wind speed: {:.2} m/s", forecast.wind);
    println!();
}
