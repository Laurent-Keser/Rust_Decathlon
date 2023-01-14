// Module import
mod controller;
mod model;
mod view;

/**
 * Prints the weather forecast of 10 Belgian cities
 */
#[tokio::main]
async fn main() {
    controller::start().await;
}
