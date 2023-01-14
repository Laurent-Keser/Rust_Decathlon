
# RUST Decathlon

Small project made for an application at DECATHLON, consisting in retrieving weather forecast data from an API.

This project is coded in RUST.


## Environment Variables

To run this project, you will need to add the following environment variable to your .env file

`API_KEY`

Said key should be retrieved from https://openweathermap.org/api


## Run Locally

Prerequisite : install Rust and cargo

Clone the project

```bash
  git clone https://github.com/Laurent-Keser/Rust_Decathlon
```

Go to the project directory

```bash
  cd weather-forecast
```

Add a .env file at the root containing the api key

```
  API_KEY="YOUR_API_KEY"
```

Build the project

```bash
  cargo build
```

Run the project

```bash
  cargo run
```

