#![allow(dead_code)]

use super::types::*;
use exitfailure::ExitFailure;
use reqwest::Url;
use crate::url_builder::UrlBuilder;
use serde::de::DeserializeOwned;

/// Finnhub API Client object.
#[derive(Debug)]
pub struct Client {
    /// API key from the Finnhub dashboard.
    pub api_key: String,
    /// Constructs urls from root, endpoints, params.
    url_bldr: UrlBuilder,
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
    pub async fn symbol_lookup(&self, query: String) -> Result<SymbolLookup, ExitFailure> {
        self.get::<SymbolLookup>(
            "search",
            &mut vec![("q", query)],
        ).await
    }

    /// Returns a list of supported stocks given the exchange.
    /// https://finnhub.io/docs/api/stock-symbols
    /// TODO: support optional params: mic, securityType, currency
    pub async fn stock_symbol(&self, exchange: String) -> Result<Vec<StockSymbol>, ExitFailure> {
        self.get::<Vec<StockSymbol>>(
            "stock/symbol",
            &mut vec![("exchange", exchange)],
        ).await
    }

    /// Returns the profile of the company specified.
    /// https://finnhub.io/docs/api/company-profile2
    /// TODO: support optional params: symbol, isin, cusip
    pub async fn company_profile2(&self, symbol: String) -> Result<CompanyProfile, ExitFailure> {
        self.get::<CompanyProfile>(
            "stock/profile2",
            &mut vec![("symbol", symbol)],
        ).await
    }

    /// Returns the latest market news in the given category.
    /// https://finnhub.io/docs/api/market-news
    /// TODO: validate category is one of [general, forex, crypto, merger]
    /// TODO: support option param: minId
    pub async fn market_news(&self, category: String) -> Result<Vec<MarketNews>, ExitFailure> {
        self.get::<Vec<MarketNews>>(
            "news",
            &mut vec![("category", category)],
        ).await
    }

    /// Returns the company news from the company specified in the given time period.
    /// https://finnhub.io/docs/api/company-news
    pub async fn company_news(
        &self,
        symbol: String,
        from: String,
        to: String,
    ) -> Result<Vec<CompanyNews>, ExitFailure> {
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
    pub async fn news_sentiment(&self, symbol: String) -> Result<NewsSentiment, ExitFailure> {
        self.get::<NewsSentiment>(
            "news-sentiment",
            &mut vec![("symbol", symbol)],
        ).await
    }

    /// Returns the specified companies peers.
    /// https://finnhub.io/docs/api/company-peers
    pub async fn peers(&self, symbol: String) -> Result<Vec<String>, ExitFailure> {
        self.get::<Vec<String>>(
            "stock/peers",
            &mut vec![("symbol", symbol)],
        ).await
    }

    /// Returns the specified company's current stock quote.
    /// https://finnhub.io/docs/api/quote
    pub async fn quote(&self, symbol: String) -> Result<CompanyQuote, ExitFailure> {
        self.get::<CompanyQuote>(
            "quote",
            &mut vec![("symbol", symbol)],
        ).await
    }

    /// Returns the basic financials of the company specified according to the given metric.
    /// https://finnhub.io/docs/api/company-basic-financials
    pub async fn basic_financials(&self, symbol: String) -> Result<BasicFinancials, ExitFailure> {
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
    ) -> Result<T, ExitFailure> {
        params.push(("token", self.api_key.clone()));
        let url_str = self.url_bldr.url(endpoint, params);
        let url = Url::parse(&url_str)?;
        let res = reqwest::get(url).await?.json::<T>().await?;
        Ok(res)
    }
}
