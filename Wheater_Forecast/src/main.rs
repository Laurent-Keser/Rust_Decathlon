//I need this because it's an extern lib and I also need to specify the dependecy in the Cargo.toml
//this one ise used to make HTTP calls
extern crate reqwest;
//this one is used to create an async block
extern crate tokio;
//this one is used because we want our API key to be in a .env file
extern crate dotenv;
//this one is used because we want to parse the JSON elements to objects
extern crate serde_json;

//main lib for HTTP requests
//in my case one example of request will be : http://api.openweathermap.org/data/2.5/weather?q=London,uk&APPID=xxxxxxxxxxxxxxxxxxxxxx
use reqwest::Client;

//lib used to run for the future response of the server in an async runtime
use tokio::runtime::Runtime;
use dotenv::dotenv;
use std::env;

fn main() {

    //getting my key from the .env if it exists
    dotenv().ok();
    let api_key = env::var("API_KEY").unwrap();

    //creating a vector (list) of somme cities. If no datas a providen you can : let v: Vec<i32> = Vec::new();
    let list_cities = vec![
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


    //for each element in the vector, print the value
    for i in &list_cities {
        //creating a new client
    let client = Client::new();
        
        //Get request to the client api
    let _resp = client.get(&format!("http://api.openweathermap.org/data/2.5/weather?q={},be&APPID={}&units=metric",i,api_key)).send();
    //The main func can't be async so we need to have an async block of code because we need to wait for the response from the server
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
    let _resp = _resp.await;
    match _resp {
        //I need to verify that the status is 200 (succes)
        Ok(forecast_result) => if forecast_result.status().is_success(){
            //Parsing the json in an object
            let result_json = forecast_result.json::<serde_json::Value>().await;
            
            match result_json {
                Ok(json) => {
                    let weather = json["weather"][0]["main"].as_str().unwrap();
                    let description = json["weather"][0]["description"].as_str().unwrap();
                    let temperature = json["main"]["temp"].as_f64().unwrap();
                    let humidity = json["main"]["humidity"].as_i64().unwrap();
                    let wind = json["wind"]["speed"].as_f64().unwrap();
                    println!("Weather in {} :", i);
                    println!("\t- Weather: {}", weather);
                    println!("\t- Description: {}", description);
                    //2 decimals
                    println!("\t- Temperature: {:.2} °C", temperature );
                    println!("\t- Humidity: {} %",humidity);
                    println!("\t- Wind speed: {:.2} m/s", wind );
                    println!();
                }
                Err(err) => {
                    println!("Error in the parsing of the json response: {:?}", err);
                }
            }



        }
        else{
            println!("Error status of the request : {}", forecast_result.status());
        }




        Err(error) => println!("Error in the request: {:?}", error),
    }
    });
    }
}
