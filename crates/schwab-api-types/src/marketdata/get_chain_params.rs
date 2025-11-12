use serde::Serialize;

/// Parameters for fetching an option chain.
#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub struct GetChainParams<'a> {
    /// The underlying symbol
    pub symbol: &'a str,
    /// Type of contracts to return (CALL, PUT, ALL)
    #[serde(rename = "contractType", skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<&'a str>,
    /// Number of strikes to return
    #[serde(rename = "strikeCount", skip_serializing_if = "Option::is_none")]
    pub strike_count: Option<i32>,
    /// Include underlying quote data
    #[serde(
        rename = "includeUnderlyingQuote",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_underlying_quote: Option<bool>,
    /// Strategy chain (e.g., SINGLE, ANALYTICAL, COVERED, VERTICAL, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<&'a str>,
    /// Strike interval for spread strategy chains
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,
    /// Strike price to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike: Option<f64>,
    /// Range of strikes (ITM, OTM, NTM, ALL, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<&'a str>,
    /// Start date for expiration filtering (yyyy-MM-dd)
    #[serde(rename = "fromDate", skip_serializing_if = "Option::is_none")]
    pub from_date: Option<&'a str>,
    /// End date for expiration filtering (yyyy-MM-dd)
    #[serde(rename = "toDate", skip_serializing_if = "Option::is_none")]
    pub to_date: Option<&'a str>,
    /// Volatility to use in calculations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volatility: Option<f64>,
    /// Underlying price for calculations
    #[serde(rename = "underlyingPrice", skip_serializing_if = "Option::is_none")]
    pub underlying_price: Option<f64>,
    /// Interest rate to use in calculations
    #[serde(rename = "interestRate", skip_serializing_if = "Option::is_none")]
    pub interest_rate: Option<f64>,
    /// Days to expiration
    #[serde(rename = "daysToExpiration", skip_serializing_if = "Option::is_none")]
    pub days_to_expiration: Option<i32>,
    /// Return only options expiring in the specified month (JAN, FEB, etc.)
    #[serde(rename = "expMonth", skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<&'a str>,
    /// Type of contracts to return (CALL, PUT, ALL)
    #[serde(rename = "optionType", skip_serializing_if = "Option::is_none")]
    pub option_type: Option<&'a str>,
}

impl<'a> GetChainParams<'a> {
    /// Create new GetChainParams with the required symbol
    pub fn new(symbol: &'a str) -> Self {
        Self {
            symbol,
            contract_type: None,
            strike_count: None,
            include_underlying_quote: None,
            strategy: None,
            interval: None,
            strike: None,
            range: None,
            from_date: None,
            to_date: None,
            volatility: None,
            underlying_price: None,
            interest_rate: None,
            days_to_expiration: None,
            exp_month: None,
            option_type: None,
        }
    }

    /// Set the contract type (CALL, PUT, ALL)
    pub fn with_contract_type(mut self, contract_type: &'a str) -> Self {
        self.contract_type = Some(contract_type);
        self
    }

    /// Set the number of strikes to return
    pub fn with_strike_count(mut self, strike_count: i32) -> Self {
        self.strike_count = Some(strike_count);
        self
    }

    /// Set whether to include underlying quote data
    pub fn with_include_underlying_quote(mut self, include_underlying_quote: bool) -> Self {
        self.include_underlying_quote = Some(include_underlying_quote);
        self
    }

    /// Set the strategy chain (SINGLE, ANALYTICAL, COVERED, VERTICAL, etc.)
    pub fn with_strategy(mut self, strategy: &'a str) -> Self {
        self.strategy = Some(strategy);
        self
    }

    /// Set the strike interval for spread strategy chains
    pub fn with_interval(mut self, interval: f64) -> Self {
        self.interval = Some(interval);
        self
    }

    /// Set the strike price to filter by
    pub fn with_strike(mut self, strike: f64) -> Self {
        self.strike = Some(strike);
        self
    }

    /// Set the range of strikes (ITM, OTM, NTM, ALL, etc.)
    pub fn with_range(mut self, range: &'a str) -> Self {
        self.range = Some(range);
        self
    }

    /// Set the start date for expiration filtering (yyyy-MM-dd)
    pub fn with_from_date(mut self, from_date: &'a str) -> Self {
        self.from_date = Some(from_date);
        self
    }

    /// Set the end date for expiration filtering (yyyy-MM-dd)
    pub fn with_to_date(mut self, to_date: &'a str) -> Self {
        self.to_date = Some(to_date);
        self
    }

    /// Set the volatility to use in calculations
    pub fn with_volatility(mut self, volatility: f64) -> Self {
        self.volatility = Some(volatility);
        self
    }

    /// Set the underlying price for calculations
    pub fn with_underlying_price(mut self, underlying_price: f64) -> Self {
        self.underlying_price = Some(underlying_price);
        self
    }

    /// Set the interest rate to use in calculations
    pub fn with_interest_rate(mut self, interest_rate: f64) -> Self {
        self.interest_rate = Some(interest_rate);
        self
    }

    /// Set the days to expiration
    pub fn with_days_to_expiration(mut self, days_to_expiration: i32) -> Self {
        self.days_to_expiration = Some(days_to_expiration);
        self
    }

    /// Set the expiration month (JAN, FEB, MAR, etc.)
    pub fn with_exp_month(mut self, exp_month: &'a str) -> Self {
        self.exp_month = Some(exp_month);
        self
    }

    /// Set the option type (CALL, PUT, ALL)
    pub fn with_option_type(mut self, option_type: &'a str) -> Self {
        self.option_type = Some(option_type);
        self
    }
}
