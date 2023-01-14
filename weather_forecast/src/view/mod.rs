use crate::model::WeatherData;

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
