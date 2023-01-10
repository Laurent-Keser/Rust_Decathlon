//I need this because it's an extern lib and I also need to specify the dependecy in the Cargo.toml
//this one ise used to make HTTP calls
extern crate reqwest;
//this one is used to create an async block
extern crate tokio;
//this one is used because we want our API key to be in a .env file
extern crate dotenv;


//main lib for HTTP requests
//in my case one example of request will be : http://api.openweathermap.org/data/2.5/weather?q=London,uk&APPID=xxxxxxxxxxxxxxxxxxxxxx
use reqwest::Client;

//lib used to run for the future response of the server in an async runtime
use tokio::runtime::Runtime;
use dotenv::dotenv;
use std::env;

fn main() {

    dotenv().ok();
    let api_key = env::var("API_KEY").unwrap();
    println!("API key: {}", api_key);
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

    println!("My favourite cities are : ");

    //for each element in the vector, print the value
    for i in &list_cities {
        println!("{i}");
    }
    //creating a new client
    let client = Client::new();

    //Get request to the client api
    let _resp = client.get(&format!("http://api.openweathermap.org/data/2.5/weather?q=London,uk&APPID={}",api_key)).send();

    //The main func can't be async so we need to have an async block of code because we need to wait for the response from the server
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let _resp = _resp.await;
        match _resp {
            Ok(_resp) => println!("Status: {}", _resp.status()),
            Err(error) => println!("Error making request: {:?}", error),
        }
    });
    
}
