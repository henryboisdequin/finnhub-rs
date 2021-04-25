#![allow(dead_code)]

use super::types::*;
use exitfailure::ExitFailure;
use reqwest::Url;
use crate::url_builder::UrlBuilder;
use serde::de::DeserializeOwned;

/// Finnhub API Client object.
#[derive(Debug, Clone)]
pub struct Client {
    /// API key from the Finnhub dashboard.
    pub api_key: String,
    /// Constructs urls from root, endpoints, params.
    pub url_bldr: UrlBuilder,
}

impl Client {
    /// Create default Finnhub Client
    pub fn new(api_key: String) -> Self {
        Client::v1(api_key)
    }

    /// Create a new V1 Finnhub Client
    pub fn v1(api_key: String) -> Self {
        Self {
            api_key,
            url_bldr: UrlBuilder::new("https://finnhub.io/api/v1"),
        }
    }

    /// Lookups a symbol in the Finnhub API
    /// https://finnhub.io/docs/api/symbol-search
    pub async fn symbol_lookup(&self, query: String) -> Result<(SymbolLookup, Url), ExitFailure> {
        self.get::<SymbolLookup>(
            "search",
            &mut vec![("q", query)],
        ).await
    }

    /// Returns a list of supported stocks given the exchange.
    /// https://finnhub.io/docs/api/stock-symbols
    pub async fn stock_symbol(&self,
                              exchange: String,
                              mic: Option<String>,
                              security_type: Option<String>,
                              currency: Option<String>) -> Result<(Vec<StockSymbol>, Url), ExitFailure> {
        let mut params = vec![("exchange", exchange)];
        Client::maybe_add(&mut params, "mic", mic);
        Client::maybe_add(&mut params, "security_type", security_type);
        Client::maybe_add(&mut params, "currency", currency);
        self.get::<Vec<StockSymbol>>(
            "stock/symbol",
            &mut params,
        ).await
    }

    /// Returns the profile of the company specified.
    /// https://finnhub.io/docs/api/company-profile2
    pub async fn company_profile2(&self, key: ProfileToParam, value: String) -> Result<(CompanyProfile, Url), ExitFailure> {
        let key = key.to_string();
        self.get::<CompanyProfile>(
            "stock/profile2",
            &mut vec![(&key, value)],
        ).await
    }

    /// Returns the latest market news in the given category.
    /// https://finnhub.io/docs/api/market-news
    pub async fn market_news(&self, category: MarketNewsCategory, min_id: Option<u64>) -> Result<(Vec<MarketNews>, Url), ExitFailure> {
        let mut params = vec![("category", category.to_string())];
        Client::maybe_add(&mut params, "minId", min_id);
        self.get::<Vec<MarketNews>>(
            "news",
            &mut params,
        ).await
    }

    /// Returns the company news from the company specified in the given time period.
    /// https://finnhub.io/docs/api/company-news
    pub async fn company_news(
        &self,
        symbol: String,
        from: String,
        to: String,
    ) -> Result<(Vec<CompanyNews>, Url), ExitFailure> {
        self.get::<Vec<CompanyNews>>(
            "company-news",
            &mut vec![
                ("symbol", symbol),
                ("from", from),
                ("to", to),
            ]
        ).await
    }

    /// Returns the latest sentiment of news of the company specified.
    /// https://finnhub.io/docs/api/news-sentiment
    pub async fn news_sentiment(&self, symbol: String) -> Result<(NewsSentiment, Url), ExitFailure> {
        self.get::<NewsSentiment>(
            "news-sentiment",
            &mut vec![("symbol", symbol)],
        ).await
    }

    /// Returns the specified companies peers.
    /// https://finnhub.io/docs/api/company-peers
    pub async fn peers(&self, symbol: String) -> Result<(Vec<String>, Url), ExitFailure> {
        self.get::<Vec<String>>(
            "stock/peers",
            &mut vec![("symbol", symbol)],
        ).await
    }

    /// Returns the specified company's current stock quote.
    /// https://finnhub.io/docs/api/quote
    pub async fn quote(&self, symbol: String) -> Result<(CompanyQuote, Url), ExitFailure> {
        self.get::<CompanyQuote>(
            "quote",
            &mut vec![("symbol", symbol)],
        ).await
    }

    /// Returns the basic financials of the company specified according to the given metric.
    /// https://finnhub.io/docs/api/company-basic-financials
    pub async fn basic_financials(&self, symbol: String) -> Result<(BasicFinancials, Url), ExitFailure> {
        self.get::<BasicFinancials>(
            "stock/metric",
            &mut vec![("symbol", symbol), ("metric", "all".into())],
        ).await
    }

    /// Compose the URL, make the request, and return the specified type.
    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &mut Vec<(&str, String)>
    ) -> Result<(T, Url), ExitFailure> {
        params.push(("token", self.api_key.clone()));
        let url_str = self.url_bldr.url(endpoint, params);
        let url = Url::parse(&url_str)?;

        #[cfg(test)]
            {
                use reqwest_mock::{ReplayClient, RecordingTarget, Client};
                use crate::utils::clean_key_from_file;

                let replay_file = self.url_bldr.replay_filename(url_str);
                let rc = ReplayClient::new(RecordingTarget::file(replay_file.clone()), Default::default());
                let response = rc.get(url.clone()).send().unwrap();
                let deserialized = serde_json::from_slice::<T>(response.body.as_slice()).unwrap();

                clean_key_from_file(replay_file, self.api_key.clone());

                Ok((deserialized, url))
            }
        #[cfg(not(test))]
            {
                let res = reqwest::get(url.clone()).await?.json::<T>().await?;
                Ok((res, url))
            }
    }

    /// If an optional parameter is Some(...), add it to the param list.
    fn maybe_add<'a, T: std::fmt::Display>(params: &mut Vec<(&'a str, String)>, param: &'a str, value: Option<T>) {
        if let Some(value) = value { params.push((param, format!("{}", value))); }
    }
}
