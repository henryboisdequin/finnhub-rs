#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! Finnhub-rs is a client for the Finnhub API implemented in Rust.
//!
//! Minimal Example:
//! ```rust
//! # // Use finnhub-rs client.
//! # use finnhub_rs::client::Client;
//! # use dotenv::dotenv;
//! # use std::env;
//! #
//! # #[tokio::main]
//! # async fn main() {
//! #   dotenv().ok();
//! #   let key = "TEST_API_KEY";
//! #   let api_key = env::var(key).expect("Key, value pair not present in .env file");
//!     // Create a new finnhub client.
//!     let client = Client::new(api_key);
//!     // Get a list of supported stocks given the exchange.
//!     let res = client.stock_symbol("US".into(), None, None, None).await.unwrap();
//!     // Print out the results.
//!     println!("{:#?}", res);
//!}
//!```

/// Finnhub-rs client which is initialized with an API key.
pub mod client;
/// All return types for all Finnhub-rs client methods.
pub mod types;
mod utils;
mod url_builder;

#[cfg(test)]
mod test {
    // `cargo t -- --nocapture` to test
    use super::client::*;
    use super::utils::*;
    use crate::types::{MarketNewsCategory, ProfileToParam};

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
        let res = client.symbol_lookup("AAPL".into()).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn stock_symbol_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.stock_symbol("US".into(), None, None, None).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn company_profile2_symbol_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.company_profile2(ProfileToParam::Symbol, "TSLA".into()).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn company_profile2_isin_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.company_profile2(ProfileToParam::ISIN, "US5949181045".into()).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn company_profile2_cusip_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.company_profile2(ProfileToParam::CUSIP, "023135106".into()).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn market_news_general_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.market_news(MarketNewsCategory::General, None).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn market_news_forex_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.market_news(MarketNewsCategory::Forex, None).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn market_news_crypto_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.market_news(MarketNewsCategory::Crypto, None).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn market_news_merger_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.market_news(MarketNewsCategory::Merger, None).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn company_news_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client
            .company_news(
                "GOOGL".into(),
                "2020-12-10".into(),
                "2021-01-10".into(),
            )
            .await
            .unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn news_sentiment_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.news_sentiment("FB".into()).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn peers_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.peers("MCD".into()).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn quote_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client.quote("TSLA".into()).await.unwrap();
        println!("{:#?}", res)
    }

    #[tokio::test]
    async fn basic_financials_test() {
        let test_api_key = get_test_api_key();
        let client = Client::new(test_api_key);
        let res = client
            .basic_financials("NFLX".into())
            .await
            .unwrap();
        println!("{:#?}", res);
    }
}
