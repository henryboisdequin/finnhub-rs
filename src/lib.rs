#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! Finnhub-rs is a client for the Finnhub API implemented in Rust.

mod client;
mod types;

#[cfg(test)]
mod test {
    use super::client::*;
    use dotenv::dotenv;
    use std::env;

    fn get_test_api_key() -> String {
        dotenv().ok();

        let key = "TEST_API_KEY";
        let test_api_key = env::var(key).expect("Key, value pair not present in .env file");

        test_api_key
    }

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
        println!("{:?}", res);
    }
}