# finnhub-rs

![License: MIT](https://img.shields.io/badge/License-MIT-red.svg)

Rust client for the [Finnhub API](https://finnhub.io/). Finnhub is a new Stock API which provides endless data for stocks, currencies, and crypto.

## Minimal Example:

```rust
// Use finnhub-rs client.
use finnhub_rs::client::Client;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let key = "TEST_API_KEY";
    let api_key = env::var(key).expect("Key, value pair not present in .env file");
    // Create a new finnhub client.
    let client = Client::new(api_key);
    // Get a list of supported stocks given the exchange.
    let res = client.stock_symbol("US".to_string()).await.unwrap();
    // Print out the results.
    println!("{:#?}", res);
}
```

## Contributing

Thanks for considering to contribute to Finnhub-rs! Please read the contributing guidelines in the [`CONTRIBUTING.md`](https://github.com/henryboisdequin/finnhub-rs/blob/main/CONTRIBUTING.md) to get started.
