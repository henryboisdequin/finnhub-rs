#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! Finnhub-rs is a client for the Finnhub API implemented in Rust.
//!
//! Minimal Example:
//! ```rust
//! // Use finnhub-rs client.
//! use finnhub_rs::client::Client;
//!
//! fn main() {
//!   // Create a new finnhub client.
//!   let client = Client::new("MY FINNHUB API KEY".to_string());
//!    // Get a list of supported stocks given the exchange.
//!    let res = client.stock_symbol("US".to_string()).await.unwrap();
//!    // Print out the results.
//!    println!("{:#?}", res);
//!}
//!```
//!

/// Finnhub-rs client which is initialized with an API key.
pub mod client;
/// All return types for all Finnhub-rs client methods.
pub mod types;
mod utils;

#[cfg(test)]
mod test {
    // `cargo t -- --nocapture` to test
    use super::client::*;
    use super::utils::*;

    #[test]
    fn create_client() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key.clone());
        assert_eq!(client.api_key, test_api_key);
    }

    #[tokio::test]
    async fn symbol_lookup_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.symbol_lookup("AAPL".to_string()).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn stock_symbol_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.stock_symbol("US".to_string()).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn company_profile2_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.company_profile2("TSLA".to_string()).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn market_news_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.market_news("business".to_string()).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn company_news_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client
            .company_news(
                "GOOGL".to_string(),
                "2020-12-10".to_string(),
                "2021-01-10".to_string(),
            )
            .await
            .unwrap();
        println!("{:#?}", res);
    }

    // #[tokio::test]
    // async fn news_sentiment_test() {
    //     // TODO: error
    //     let test_api_key = get_test_api_key();
    //     let client = Client::new(test_api_key);
    //     let res = client.news_sentiment("FB".to_string()).await.unwrap();
    //     println!("{:#?}", res);
    // }

    #[tokio::test]
    async fn peers_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.peers("MCD".to_string()).await.unwrap();
        println!("{:#?}", res);
    }

    // #[tokio::test]
    // async fn basic_financials_test() {
    //     // TODO: error
    //     let test_api_key = get_test_api_key();
    //     let client = Client::new(test_api_key);
    //     let res = client
    //         .basic_financials("NFLX".to_string(), "all".to_string())
    //         .await
    //         .unwrap();
    //     println!("{:#?}", res);
    // }
}
