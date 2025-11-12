use serde::{Deserialize, Serialize};

/// Market data information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FundamentalInst {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "high52", skip_serializing_if = "Option::is_none")]
    pub high52: Option<f64>,
    #[serde(rename = "low52", skip_serializing_if = "Option::is_none")]
    pub low52: Option<f64>,
    #[serde(rename = "dividendAmount", skip_serializing_if = "Option::is_none")]
    pub dividend_amount: Option<f64>,
    #[serde(rename = "dividendYield", skip_serializing_if = "Option::is_none")]
    pub dividend_yield: Option<f64>,
    #[serde(rename = "dividendDate", skip_serializing_if = "Option::is_none")]
    pub dividend_date: Option<String>,
    #[serde(rename = "peRatio", skip_serializing_if = "Option::is_none")]
    pub pe_ratio: Option<f64>,
    #[serde(rename = "pegRatio", skip_serializing_if = "Option::is_none")]
    pub peg_ratio: Option<f64>,
    #[serde(rename = "pbRatio", skip_serializing_if = "Option::is_none")]
    pub pb_ratio: Option<f64>,
    #[serde(rename = "prRatio", skip_serializing_if = "Option::is_none")]
    pub pr_ratio: Option<f64>,
    #[serde(rename = "pcfRatio", skip_serializing_if = "Option::is_none")]
    pub pcf_ratio: Option<f64>,
    #[serde(rename = "grossMarginTTM", skip_serializing_if = "Option::is_none")]
    pub gross_margin_ttm: Option<f64>,
    #[serde(rename = "grossMarginMRQ", skip_serializing_if = "Option::is_none")]
    pub gross_margin_mrq: Option<f64>,
    #[serde(rename = "netProfitMarginTTM", skip_serializing_if = "Option::is_none")]
    pub net_profit_margin_ttm: Option<f64>,
    #[serde(rename = "netProfitMarginMRQ", skip_serializing_if = "Option::is_none")]
    pub net_profit_margin_mrq: Option<f64>,
    #[serde(rename = "operatingMarginTTM", skip_serializing_if = "Option::is_none")]
    pub operating_margin_ttm: Option<f64>,
    #[serde(rename = "operatingMarginMRQ", skip_serializing_if = "Option::is_none")]
    pub operating_margin_mrq: Option<f64>,
    #[serde(rename = "returnOnEquity", skip_serializing_if = "Option::is_none")]
    pub return_on_equity: Option<f64>,
    #[serde(rename = "returnOnAssets", skip_serializing_if = "Option::is_none")]
    pub return_on_assets: Option<f64>,
    #[serde(rename = "returnOnInvestment", skip_serializing_if = "Option::is_none")]
    pub return_on_investment: Option<f64>,
    #[serde(rename = "quickRatio", skip_serializing_if = "Option::is_none")]
    pub quick_ratio: Option<f64>,
    #[serde(rename = "currentRatio", skip_serializing_if = "Option::is_none")]
    pub current_ratio: Option<f64>,
    #[serde(rename = "interestCoverage", skip_serializing_if = "Option::is_none")]
    pub interest_coverage: Option<f64>,
    #[serde(rename = "totalDebtToCapital", skip_serializing_if = "Option::is_none")]
    pub total_debt_to_capital: Option<f64>,
    #[serde(rename = "ltDebtToEquity", skip_serializing_if = "Option::is_none")]
    pub lt_debt_to_equity: Option<f64>,
    #[serde(rename = "totalDebtToEquity", skip_serializing_if = "Option::is_none")]
    pub total_debt_to_equity: Option<f64>,
    #[serde(rename = "epsTTM", skip_serializing_if = "Option::is_none")]
    pub eps_ttm: Option<f64>,
    #[serde(
        rename = "epsChangePercentTTM",
        skip_serializing_if = "Option::is_none"
    )]
    pub eps_change_percent_ttm: Option<f64>,
    #[serde(rename = "epsChangeYear", skip_serializing_if = "Option::is_none")]
    pub eps_change_year: Option<f64>,
    #[serde(rename = "epsChange", skip_serializing_if = "Option::is_none")]
    pub eps_change: Option<f64>,
    #[serde(rename = "revChangeYear", skip_serializing_if = "Option::is_none")]
    pub rev_change_year: Option<f64>,
    #[serde(rename = "revChangeTTM", skip_serializing_if = "Option::is_none")]
    pub rev_change_ttm: Option<f64>,
    #[serde(rename = "revChangeIn", skip_serializing_if = "Option::is_none")]
    pub rev_change_in: Option<f64>,
    #[serde(rename = "sharesOutstanding", skip_serializing_if = "Option::is_none")]
    pub shares_outstanding: Option<f64>,
    #[serde(rename = "marketCapFloat", skip_serializing_if = "Option::is_none")]
    pub market_cap_float: Option<f64>,
    #[serde(rename = "marketCap", skip_serializing_if = "Option::is_none")]
    pub market_cap: Option<f64>,
    #[serde(rename = "bookValuePerShare", skip_serializing_if = "Option::is_none")]
    pub book_value_per_share: Option<f64>,
    #[serde(rename = "shortIntToFloat", skip_serializing_if = "Option::is_none")]
    pub short_int_to_float: Option<f64>,
    #[serde(rename = "shortIntDayToCover", skip_serializing_if = "Option::is_none")]
    pub short_int_day_to_cover: Option<f64>,
    #[serde(rename = "divGrowthRate3Year", skip_serializing_if = "Option::is_none")]
    pub div_growth_rate3_year: Option<f64>,
    #[serde(rename = "dividendPayAmount", skip_serializing_if = "Option::is_none")]
    pub dividend_pay_amount: Option<f64>,
    #[serde(rename = "dividendPayDate", skip_serializing_if = "Option::is_none")]
    pub dividend_pay_date: Option<String>,
    #[serde(rename = "beta", skip_serializing_if = "Option::is_none")]
    pub beta: Option<f64>,
    #[serde(rename = "vol1DayAvg", skip_serializing_if = "Option::is_none")]
    pub vol1_day_avg: Option<f64>,
    #[serde(rename = "vol10DayAvg", skip_serializing_if = "Option::is_none")]
    pub vol10_day_avg: Option<f64>,
    #[serde(rename = "vol3MonthAvg", skip_serializing_if = "Option::is_none")]
    pub vol3_month_avg: Option<f64>,
    #[serde(rename = "avg10DaysVolume", skip_serializing_if = "Option::is_none")]
    pub avg10_days_volume: Option<i64>,
    #[serde(rename = "avg1DayVolume", skip_serializing_if = "Option::is_none")]
    pub avg1_day_volume: Option<i64>,
    #[serde(rename = "avg3MonthVolume", skip_serializing_if = "Option::is_none")]
    pub avg3_month_volume: Option<i64>,
    #[serde(rename = "declarationDate", skip_serializing_if = "Option::is_none")]
    pub declaration_date: Option<String>,
    #[serde(rename = "dividendFreq", skip_serializing_if = "Option::is_none")]
    pub dividend_freq: Option<i32>,
    #[serde(rename = "eps", skip_serializing_if = "Option::is_none")]
    pub eps: Option<f64>,
    #[serde(rename = "corpactionDate", skip_serializing_if = "Option::is_none")]
    pub corpaction_date: Option<String>,
    #[serde(rename = "dtnVolume", skip_serializing_if = "Option::is_none")]
    pub dtn_volume: Option<i64>,
    #[serde(
        rename = "nextDividendPayDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_dividend_pay_date: Option<String>,
    #[serde(rename = "nextDividendDate", skip_serializing_if = "Option::is_none")]
    pub next_dividend_date: Option<String>,
    #[serde(rename = "fundLeverageFactor", skip_serializing_if = "Option::is_none")]
    pub fund_leverage_factor: Option<f64>,
    #[serde(rename = "fundStrategy", skip_serializing_if = "Option::is_none")]
    pub fund_strategy: Option<String>,
}

