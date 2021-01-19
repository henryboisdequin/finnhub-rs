use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolLookup {
    count: usize,
    result: Vec<Results>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    description: String,
    displaySymbol: String,
    symbol: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockSymbol {
    description: String,
    displaySymbol: String,
    symbol: String,
    r#type: String,
    mic: String,
    figi: String,
    currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyProfile {
    country: String,
    currency: String,
    exchange: String,
    ipo: String,
    name: String,
    phone: String,
    shareOutstanding: f64,
    ticker: String,
    weburl: String,
    logo: String,
    finnhubIndustry: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketNews {
    category: String,
    datetime: u128,
    headline: String,
    id: u128,
    image: String,
    related: String,
    source: String,
    summary: String,
    url: String,
}
