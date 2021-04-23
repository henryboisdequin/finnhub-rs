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
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockSymbol {
    pub description: String,
    pub displaySymbol: String,
    pub symbol: String,
    #[serde(rename = "type")]
    pub _type: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsSentiment {
    pub buzz: Buzz,
    pub companyNewsScore: f64,
    pub sectorAverageBullishPercent: f64,
    pub sectorAverageNewsScore: f64,
    pub sentiment: Sentiment,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Buzz {
    pub articlesInLastWeek: usize,
    pub buzz: f64,
    pub weeklyAverage: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sentiment {
    pub bearishPercent: f64,
    pub bullishPercent: f64,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicFinancials {
    pub series: Series,
    pub metric: Metric,
    pub metricType: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Series {
    pub annual: Annual,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annual {
    pub cashRatio: Vec<PeriodPlusV>,
    pub currentRatio: Vec<PeriodPlusV>,
    pub ebitPerShare: Vec<PeriodPlusV>,
    pub eps: Vec<PeriodPlusV>,
    pub grossMargin: Vec<PeriodPlusV>,
    pub longtermDebtTotalAsset: Vec<PeriodPlusV>,
    pub longtermDebtTotalCapital: Vec<PeriodPlusV>,
    pub longtermDebtTotalEquity: Vec<PeriodPlusV>,
    pub netDebtToTotalCapital: Vec<PeriodPlusV>,
    pub netDebtToTotalEquity: Vec<PeriodPlusV>,
    pub netMargin: Vec<PeriodPlusV>,
    pub operatingMargin: Vec<PeriodPlusV>,
    pub pretaxMargin: Vec<PeriodPlusV>,
    pub salesPerShare: Vec<PeriodPlusV>,
    pub sgaToSale: Vec<PeriodPlusV>,
    pub totalDebtToEquity: Vec<PeriodPlusV>,
    pub totalDebtToTotalAsset: Vec<PeriodPlusV>,
    pub totalDebtToTotalCapital: Vec<PeriodPlusV>,
    pub totalRatio: Vec<PeriodPlusV>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeriodPlusV {
    pub period: String,
    pub v: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metric {
    #[serde(rename = "10DayAverageTradingVolume")]
    pub averageTradingVolume_10Day: Option<f64>,
    #[serde(rename = "13WeekPriceReturnDaily")]
    pub priceReturnDaily_13Week: Option<f64>,
    #[serde(rename = "26WeekPriceReturnDaily")]
    pub priceReturnDaily_26Week: Option<f64>,
    #[serde(rename = "3MonthAverageTradingVolume")]
    pub averageTradingVolume_3Month: Option<f64>,
    #[serde(rename = "52WeekHigh")]
    pub high_52Week: Option<f64>,
    #[serde(rename = "52WeekHighDate")]
    pub high_date_52Week: Option<String>,
    #[serde(rename = "52WeekLow")]
    pub low_52Week: Option<f64>,
    #[serde(rename = "52WeekLowDate")]
    pub low_date_52Week: Option<String>,
    #[serde(rename = "52WeekPriceReturnDaily")]
    pub priceReturnDaily_52Week: Option<f64>,
    #[serde(rename = "5DayPriceReturnDaily")]
    pub priceReturnDaily_5Day: Option<f64>,
    pub assetTurnoverAnnual: Option<f64>,
    pub assetTurnoverTTM: Option<f64>,
    pub beta: Option<f64>,
    pub bookValuePerShareAnnual: Option<f64>,
    pub bookValuePerShareQuarterly: Option<f64>,
    pub bookValueShareGrowth5Y: Option<f64>,
    pub capitalSpendingGrowth5Y: Option<f64>,
    pub cashFlowPerShareAnnual: Option<f64>,
    pub cashFlowPerShareTTM: Option<f64>,
    pub cashPerSharePerShareAnnual: Option<f64>,
    pub cashPerSharePerShareQuarterly: Option<f64>,
    pub currentDividendYieldTTM: Option<f64>,
    #[serde(rename = "currentEv/freeCashFlowAnnual")]
    pub currentEv_freeCashFlowAnnual: Option<f64>,
    #[serde(rename = "currentEv/freeCashFlowTTM")]
    pub currentEv_freeCashFlowTTM: Option<f64>,
    pub currentRatioAnnual: Option<f64>,
    pub currentRatioQuarterly: Option<f64>,
    pub dividendGrowthRate5Y: Option<f64>,
    pub dividendPerShare5Y: Option<f64>,
    pub dividendPerShareAnnual: Option<f64>,
    pub dividendYield5Y: Option<f64>,
    pub dividendYieldIndicatedAnnual: Option<f64>,
    pub dividendsPerShareTTM: Option<f64>,
    pub ebitdPerShareTTM: Option<f64>,
    pub ebitdaCagr5Y: Option<f64>,
    pub ebitdaInterimCagr5Y: Option<f64>,
    pub epsBasicExclExtraItemsAnnual: Option<f64>,
    pub epsBasicExclExtraItemsTTM: Option<f64>,
    pub epsExclExtraItemsAnnual: Option<f64>,
    pub epsExclExtraItemsTTM: Option<f64>,
    pub epsGrowth3Y: Option<f64>,
    pub epsGrowth5Y: Option<f64>,
    pub epsGrowthQuarterlyYoy: Option<f64>,
    pub epsGrowthTTMYoy: Option<f64>,
    pub epsInclExtraItemsAnnual: Option<f64>,
    pub epsInclExtraItemsTTM: Option<f64>,
    pub epsNormalizedAnnual: Option<f64>,
    pub focfCagr5Y: Option<f64>,
    pub freeCashFlowAnnual: Option<f64>,
    pub freeCashFlowPerShareTTM: Option<f64>,
    pub freeCashFlowTTM: Option<f64>,
    #[serde(rename = "freeOperatingCashFlow/revenue5Y")]
    pub freeOperatingCashFlow_revenue5Y: Option<f64>,
    #[serde(rename = "freeOperatingCashFlow/revenueTTM")]
    pub freeOperatingCashFlow_revenueTTM: Option<f64>,
    pub grossMargin5Y: Option<f64>,
    pub grossMarginAnnual: Option<f64>,
    pub grossMarginTTM: Option<f64>,
    pub inventoryTurnoverAnnual: Option<f64>,
    pub inventoryTurnoverTTM: Option<f64>,
    #[serde(rename = "longTermDebt/equityAnnual")]
    pub longTermDebt_equityAnnual: Option<f64>,
    #[serde(rename = "longTermDebt/equityQuarterly")]
    pub longTermDebt_equityQuarterly: Option<f64>,
    pub marketCapitalization: Option<f64>,
    pub monthToDatePriceReturnDaily: Option<f64>,
    pub netDebtAnnual: Option<f64>,
    pub netDebtInterim: Option<f64>,
    pub netIncomeEmployeeAnnual: Option<f64>,
    pub netIncomeEmployeeTTM: Option<f64>,
    pub netInterestCoverageAnnual: Option<f64>,
    pub netInterestCoverageTTM: Option<f64>,
    pub netMarginGrowth5Y: Option<f64>,
    pub netProfitMargin5Y: Option<f64>,
    pub netProfitMarginAnnual: Option<f64>,
    pub netProfitMarginTTM: Option<f64>,
    pub operatingMargin5Y: Option<f64>,
    pub operatingMarginAnnual: Option<f64>,
    pub operatingMarginTTM: Option<f64>,
    pub payoutRatioAnnual: Option<f64>,
    pub payoutRatioTTM: Option<f64>,
    pub pbAnnual: Option<f64>,
    pub pbQuarterly: Option<f64>,
    pub pcfShareTTM: Option<f64>,
    pub peBasicExclExtraTTM: Option<f64>,
    pub peExclExtraAnnual: Option<f64>,
    pub peExclExtraHighTTM: Option<f64>,
    pub peExclExtraTTM: Option<f64>,
    pub peExclLowTTM: Option<f64>,
    pub peInclExtraTTM: Option<f64>,
    pub peNormalizedAnnual: Option<f64>,
    pub pfcfShareAnnual: Option<f64>,
    pub pfcfShareTTM: Option<f64>,
    pub pretaxMargin5Y: Option<f64>,
    pub pretaxMarginAnnual: Option<f64>,
    pub pretaxMarginTTM: Option<f64>,
    #[serde(rename = "priceRelativeToS\u{0026}P50013Week")]
    pub priceRelativeToSP500_13Week: Option<f64>,
    #[serde(rename = "priceRelativeToS\u{0026}P50026Week")]
    pub priceRelativeToSP50026Week: Option<f64>,
    #[serde(rename = "priceRelativeToS\u{0026}P5004Week")]
    pub priceRelativeToSP500_4Week: Option<f64>,
    #[serde(rename = "priceRelativeToS\u{0026}P50052Week")]
    pub priceRelativeToSP500_52Week: Option<f64>,
    #[serde(rename = "priceRelativeToS\u{0026}P500Ytd")]
    pub priceRelativeToSP500_Ytd: Option<f64>,
    pub psAnnual: Option<f64>,
    pub psTTM: Option<f64>,
    pub ptbvAnnual: Option<f64>,
    pub ptbvQuarterly: Option<f64>,
    pub quickRatioAnnual: Option<f64>,
    pub quickRatioQuarterly: Option<f64>,
    pub receivablesTurnoverAnnual: Option<f64>,
    pub receivablesTurnoverTTM: Option<f64>,
    pub revenueEmployeeAnnual: Option<f64>,
    pub revenueEmployeeTTM: Option<f64>,
    pub revenueGrowth3Y: Option<f64>,
    pub revenueGrowth5Y: Option<f64>,
    pub revenueGrowthQuarterlyYoy: Option<f64>,
    pub revenueGrowthTTMYoy: Option<f64>,
    pub revenuePerShareAnnual: Option<f64>,
    pub revenuePerShareTTM: Option<f64>,
    pub revenueShareGrowth5Y: Option<f64>,
    pub roaRfy: Option<f64>,
    pub roaa5Y: Option<f64>,
    pub roae5Y: Option<f64>,
    pub roaeTTM: Option<f64>,
    pub roeRfy: Option<f64>,
    pub roeTTM: Option<f64>,
    pub roi5Y: Option<f64>,
    pub roiAnnual: Option<f64>,
    pub roiTTM: Option<f64>,
    pub tangibleBookValuePerShareAnnual: Option<f64>,
    pub tangibleBookValuePerShareQuarterly: Option<f64>,
    pub tbvCagr5Y: Option<f64>,
    #[serde(rename = "totalDebt/totalEquityAnnual")]
    pub totalDebt_totalEquityAnnual: Option<f64>,
    #[serde(rename = "totalDebt/totalEquityQuarterly")]
    pub totalDebt_totalEquityQuarterly: Option<f64>,
    pub totalDebtCagr5Y: Option<f64>,
    pub yearToDatePriceReturnDaily: Option<f64>,
}

#[derive(Debug)]
pub enum MarketNewsCategory {
    General,
    Forex,
    Crypto,
    Merger,
}

impl From<MarketNewsCategory> for String {
    fn from(category: MarketNewsCategory) -> String {
        match category {
            MarketNewsCategory::General => "general",
            MarketNewsCategory::Forex => "forex",
            MarketNewsCategory::Crypto => "crypto",
            MarketNewsCategory::Merger => "merger",
        }.to_string()
    }
}

impl std::fmt::Display for MarketNewsCategory {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(match *self {
            MarketNewsCategory::General => "general",
            MarketNewsCategory::Forex => "forex",
            MarketNewsCategory::Crypto => "crypto",
            MarketNewsCategory::Merger => "merger",
        })?;
        Ok(())
    }
}
