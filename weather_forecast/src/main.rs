// Module import
mod forecast;
mod view;

/**
 * Prints the weather forecast of 10 Belgian cities
 */
#[tokio::main]
async fn main() {
    view::display_menu().await;
}
