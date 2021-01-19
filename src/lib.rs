#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! Finnhub-rs is a client for the Finnhub API implemented in Rust.

mod client;
mod types;

#[cfg(test)]
mod test {
    use super::client::*;
    use super::types::*;
    use exitfailure::ExitFailure;

    #[test]
    fn test_create_client() {
        let client = Client::new("API_KEY".to_string());
        assert_eq!(client.api_key, "API_KEY".to_string());
    }
}
