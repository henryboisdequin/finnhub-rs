# finnhub-rs

![License: MIT](https://img.shields.io/badge/License-MIT-red.svg)

Rust client for the <a href="https://finnhub.io/">Finnhub API</a>. Finnhub is a new Stock API which provides endless data for stocks, currencies, and crypto.

## Minimal Example:

```rust
// Use finnhub-rs client.
use finnhub_rs::client::Client;

fn main() {
    // Create a new finnhub client.
    let client = Client::v1("MY FINNHUB API KEY".to_string());
    // Get a list of supported stocks given the exchange.
    let res = client.stock_symbol("US".to_string()).await.unwrap();
    // Print out the results.
    println!("{:#?}", res);
}
```
