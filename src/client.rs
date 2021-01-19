#![allow(dead_code)]

use super::types::*;
use exitfailure::ExitFailure;
use reqwest::Url;

/// Finnhub API Client object.
#[derive(Debug)]
pub struct Client {
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

    pub async fn stock_symbol(self, exchange: String) -> Result<Vec<StockSymbol>, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/stock/symbol?exchange={}&token={}",
            exchange, self.api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<Vec<StockSymbol>>().await?;

        Ok(res)
    }

    pub async fn company_profile2(self, symbol: String) -> Result<CompanyProfile, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/stock/profile2?symbol={}&token={}",
            symbol, self.api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<CompanyProfile>().await?;

        Ok(res)
    }

    pub async fn market_news(self, category: String) -> Result<Vec<MarketNews>, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/news?category={}&token={}",
            category, self.api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<Vec<MarketNews>>().await?;

        Ok(res)
    }

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

    pub async fn news_sentiment(self, symbol: String) -> Result<NewsSentiment, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/news-sentiment?symbol={}&token={}",
            symbol, self.api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<NewsSentiment>().await?;

        Ok(res)
    }
}
