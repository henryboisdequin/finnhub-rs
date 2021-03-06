#![allow(non_snake_case, missing_docs)]

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolLookup {
    pub count: usize,
    pub result: Vec<Results>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    pub description: String,
    pub displaySymbol: String,
    pub symbol: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockSymbol {
    pub description: String,
    pub displaySymbol: String,
    pub symbol: String,
    pub r#type: String,
    pub mic: String,
    pub figi: String,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyProfile {
    pub country: String,
    pub currency: String,
    pub exchange: String,
    pub ipo: String,
    pub name: String,
    pub phone: String,
    pub shareOutstanding: f64,
    pub ticker: String,
    pub weburl: String,
    pub logo: String,
    pub finnhubIndustry: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketNews {
    pub category: String,
    pub datetime: u128,
    pub headline: String,
    pub id: u128,
    pub image: String,
    pub related: String,
    pub source: String,
    pub summary: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyNews {
    pub category: String,
    pub datetime: u128,
    pub headline: String,
    pub id: u128,
    pub image: String,
    pub related: String,
    pub source: String,
    pub summary: String,
    pub url: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct NewsSentiment {
//     pub buzz: Buzz,
//     pub companyNewsScore: i128,
//     pub sectorAverageBullishPercent: i128,
//     pub sectorAverageNewsScore: i128,
//     pub sentiment: Sentiment,
//     pub symbol: String,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Buzz {
    pub articlesInLastWeek: usize,
    pub buzz: i128,
    pub weeklyAverage: isize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sentiment {
    pub bearishPercent: usize,
    pub bullishPercent: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyQuote {
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub o: f64,
    pub pc: f64,
    pub t: i128,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct BasicFinancials {
//     pub series: Series,
//     pub metric: Metric,
//     pub metricType: String,
//     pub symbol: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Series {
//     pub annual: Annual,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Annual {
//     pub currentRatio: Vec<PeriodPlusV>,
//     pub salesPerShare: Vec<PeriodPlusV>,
//     pub netMargin: Vec<PeriodPlusV>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct PeriodPlusV {
//     pub period: String,
//     pub v: f64,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Metric {
//     pub _10DayAverageTradingVolume: f64,
//     pub _52WeekHigh: f64,
//     pub _52WeekLow: f64,
//     pub _52WeekLowDate: String,
//     pub _52WeekPriceReturnDaily: f64,
//     pub beta: f64,
// }
