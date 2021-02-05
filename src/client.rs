#![allow(dead_code)]

use super::types::*;
use exitfailure::ExitFailure;
use reqwest::Url;

/// Finnhub API Client object.
#[derive(Debug)]
pub struct Client {
    /// API key from the Finnhub dashboard.
    pub api_key: String,
}

impl Client {
    /// Creates a new Finnhub Client
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    /// Lookups a symbol in the Finnhub API
    pub async fn symbol_lookup(self, query: String) -> Result<SymbolLookup, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/search?q={}&token={}",
            query, self.api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<SymbolLookup>().await?;

        Ok(res)
    }

    /// Returns a list of supported stocks given the exchange.
    pub async fn stock_symbol(self, exchange: String) -> Result<Vec<StockSymbol>, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/stock/symbol?exchange={}&token={}",
            exchange, self.api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<Vec<StockSymbol>>().await?;

        Ok(res)
    }

    /// Returns the profile of the company specified.
    pub async fn company_profile2(self, symbol: String) -> Result<CompanyProfile, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/stock/profile2?symbol={}&token={}",
            symbol, self.api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<CompanyProfile>().await?;

        Ok(res)
    }

    /// Returns the latest market news in the given category.
    pub async fn market_news(self, category: String) -> Result<Vec<MarketNews>, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/news?category={}&token={}",
            category, self.api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<Vec<MarketNews>>().await?;

        Ok(res)
    }

    /// Returns the company news from the company specified in the given time period.
    pub async fn company_news(
        self,
        symbol: String,
        from: String,
        to: String,
    ) -> Result<Vec<CompanyNews>, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/company-news?symbol={}&from={}&to={}&token={}",
            symbol, from, to, self.api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<Vec<CompanyNews>>().await?;

        Ok(res)
    }

    // /// Returns the latest sentiment of news of the company specified.
    // pub async fn news_sentiment(self, symbol: String) -> Result<NewsSentiment, ExitFailure> {
    //     let url = format!(
    //         "https://finnhub.io/api/v1/news-sentiment?symbol={}&token={}",
    //         symbol, self.api_key
    //     );

    //     let url = Url::parse(&*url)?;
    //     let res = reqwest::get(url).await?.json::<NewsSentiment>().await?;

    //     Ok(res)
    // }

    /// Returns the specified companies peers.
    pub async fn peers(self, symbol: String) -> Result<Vec<String>, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/stock/peers?symbol={}&token={}",
            symbol, self.api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<Vec<String>>().await?;

        Ok(res)
    }

    /// Returns the specified company's current stock quote.
    pub async fn quote(self, symbol: String) -> Result<CompanyQuote, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, self.api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<CompanyQuote>().await?;

        Ok(res)
    }

    // /// Returns the basic financials of the company specified according to the given metric.
    // pub async fn basic_financials(
    //     self,
    //     symbol: String,
    //     metric: String,
    // ) -> Result<BasicFinancials, ExitFailure> {
    //     let url = format!(
    //         "https://finnhub.io/api/v1/stock/metric?symbol={}&metric={}&token={}",
    //         symbol, metric, self.api_key
    //     );

    //     let url = Url::parse(&*url)?;
    //     let res = reqwest::get(url).await?.json::<BasicFinancials>().await?;

    //     Ok(res)
    // }
}
