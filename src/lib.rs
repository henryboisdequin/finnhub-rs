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
/// Utility functions related to testing and acquiring api keys.
pub mod utils;
/// Helper for generating urls and test filenames.
pub mod url_builder;

/// If you are adding a new network test, you'll first need to test with a real api
/// key to generate a response. After your test is passing, replace your api key with
/// the dummy api key and check in the replay file that was generated in test folder.

#[cfg(test)]
mod test {
    // `cargo t -- --nocapture` to test
    use super::client::*;
    use super::utils::*;
    use super::types::*;


    /// Macro to generate client and run api test. Helps remove a bunch of boilerplate.
    macro_rules! api_test {
        ($api_fn:expr) => {
            let test_api_key = get_dummy_api_key();
            let client = Client::new(test_api_key);
            let (obj, url) = $api_fn(client.clone()).await.unwrap();

            let expected_filename = client.url_bldr.expected_filename(url.to_string());
            let expected = load_expected_from_replay_filename(expected_filename);
            let actual = format!("{:#?}", obj);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn create_client() {
        let test_api_key = get_dummy_api_key();
        let client = Client::new(test_api_key.clone());
        assert_eq!(client.api_key, test_api_key);
    }

    #[tokio::test]
    async fn symbol_lookup_test() {
        api_test!(|client: Client| async move {
            client.symbol_lookup("AAPL".into()).await
        });
    }

    #[tokio::test]
    async fn stock_symbol_test() {
        let test_api_key = get_dummy_api_key();
        let client = Client::new(test_api_key);
        let (obj, _url) = client.stock_symbol("US".into(), None, None, None).await.unwrap();

        // TODO: Figure out a real test. Also, this test always seems to terminate before completion,
        //       so figure that out.
        println!("{:#?}", obj);
    }

    #[tokio::test]
    async fn company_profile2_symbol_test() {
        api_test!(|client: Client| async move {
            client.company_profile2(ProfileToParam::Symbol, "TSLA".into()).await
        });
    }

    #[tokio::test]
    async fn company_profile2_isin_test() {
        api_test!(|client: Client| async move {
            client.company_profile2(ProfileToParam::ISIN, "US5949181045".into()).await
        });
    }

    #[tokio::test]
    async fn company_profile2_cusip_test() {
        api_test!(|client: Client| async move {
            client.company_profile2(ProfileToParam::CUSIP, "023135106".into()).await
        });
    }

    #[tokio::test]
    async fn market_news_general_test() {
        api_test!(|client: Client| async move {
            client.market_news(MarketNewsCategory::General, None).await
        });
    }

    #[tokio::test]
    async fn market_news_forex_test() {
        api_test!(|client: Client| async move {
            client.market_news(MarketNewsCategory::Forex, None).await
        });
    }

    #[tokio::test]
    async fn market_news_crypto_test() {
        api_test!(|client: Client| async move {
            client.market_news(MarketNewsCategory::Crypto, None).await
        });
    }

    #[tokio::test]
    async fn market_news_merger_test() {
        api_test!(|client: Client| async move {
            client.market_news(MarketNewsCategory::Merger, None).await
        });
    }

    #[tokio::test]
    async fn company_news_test() {
        api_test!(|client: Client| async move {
            client.company_news(
                "GOOGL".into(),
                "2020-12-10".into(),
                "2021-01-10".into(),
            )
            .await
        });
    }

    #[tokio::test]
    async fn news_sentiment_test() {
        api_test!(|client: Client| async move {
            client.news_sentiment("FB".into()).await
        });
    }

    #[tokio::test]
    async fn peers_test() {
        api_test!(|client: Client| async move {
            client.peers("MCD".into()).await
        });
    }

    #[tokio::test]
    async fn quote_test() {
        api_test!(|client: Client| async move {
            client.quote("TSLA".into()).await
        });
    }

    #[tokio::test]
    async fn basic_financials_test() {
        api_test!(|client: Client| async move {
            client.basic_financials("NFLX".into()).await
        });
    }
}
