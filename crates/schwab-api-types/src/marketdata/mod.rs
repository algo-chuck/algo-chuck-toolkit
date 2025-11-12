//! Market data types for the Schwab API.
//!
//! This module contains all types used in the Schwab Market Data API including:
//! - Quotes for various asset types (equities, options, futures, forex, etc.)
//! - Option chains and expiration data
//! - Price history and candlestick data
//! - Market hours information
//! - Instrument reference data
//! - Market movers and screening
//!
//! Types are organized into response types (received from API), request types (sent to API),
//! enums (shared types), and parameter structs (for API calls).

// ============================================================================
// Response Types - Returned by API operations
// ============================================================================

pub mod equity_response;
pub use equity_response::EquityResponse;

pub mod forex_response;
pub use forex_response::ForexResponse;

pub mod future_response;
pub use future_response::FutureResponse;

pub mod future_option_response;
pub use future_option_response::FutureOptionResponse;

pub mod index_response;
pub use index_response::IndexResponse;

pub mod mutual_fund_response;
pub use mutual_fund_response::MutualFundResponse;

pub mod option_response;
pub use option_response::OptionResponse;

pub mod error_response;
pub use error_response::ErrorResponse;

pub mod get_instruments_200_response;
pub use get_instruments_200_response::GetInstruments200Response;

pub mod get_movers_200_response;
pub use get_movers_200_response::GetMovers200Response;

// ============================================================================
// Quote Types
// ============================================================================

pub mod quote_equity;
pub use quote_equity::QuoteEquity;

pub mod quote_error;
pub use quote_error::QuoteError;

pub mod quote_forex;
pub use quote_forex::QuoteForex;

pub mod quote_future;
pub use quote_future::QuoteFuture;

pub mod quote_future_option;
pub use quote_future_option::QuoteFutureOption;

pub mod quote_index;
pub use quote_index::QuoteIndex;

pub mod quote_mutual_fund;
pub use quote_mutual_fund::QuoteMutualFund;

pub mod quote_option;
pub use quote_option::QuoteOption;

pub mod quote_request;
pub use quote_request::QuoteRequest;

pub mod quote_response_object;
pub use quote_response_object::QuoteResponseObject;

// ============================================================================
// Reference Data Types
// ============================================================================

pub mod reference_equity;
pub use reference_equity::ReferenceEquity;

pub mod reference_forex;
pub use reference_forex::ReferenceForex;

pub mod reference_future;
pub use reference_future::ReferenceFuture;

pub mod reference_future_option;
pub use reference_future_option::ReferenceFutureOption;

pub mod reference_index;
pub use reference_index::ReferenceIndex;

pub mod reference_mutual_fund;
pub use reference_mutual_fund::ReferenceMutualFund;

pub mod reference_option;
pub use reference_option::ReferenceOption;

// ============================================================================
// Option Chain Types
// ============================================================================

pub mod option_chain;
pub use option_chain::OptionChain;

pub mod option_contract;
pub use option_contract::OptionContract;

pub mod option_deliverables;
pub use option_deliverables::OptionDeliverables;

pub mod expiration;
pub use expiration::Expiration;

pub mod expiration_chain;
pub use expiration_chain::ExpirationChain;

pub mod underlying;
pub use underlying::Underlying;

// ============================================================================
// Price History & Candles
// ============================================================================

pub mod candle;
pub use candle::Candle;

pub mod candle_list;
pub use candle_list::CandleList;

// ============================================================================
// Market Information
// ============================================================================

pub mod hours;
pub use hours::Hours;

pub mod interval;
pub use interval::Interval;

pub mod extended_market;
pub use extended_market::ExtendedMarket;

pub mod regular_market;
pub use regular_market::RegularMarket;

pub mod screener;
pub use screener::Screener;

// ============================================================================
// Instrument Types
// ============================================================================

pub mod instrument;
pub use instrument::Instrument;

pub mod instrument_response;
pub use instrument_response::InstrumentResponse;

pub mod fundamental;
pub use fundamental::Fundamental;

pub mod fundamental_inst;
pub use fundamental_inst::FundamentalInst;

pub mod bond;
pub use bond::Bond;

// ============================================================================
// Enums - Shared types for requests and responses
// ============================================================================

pub mod asset_main_type;
pub use asset_main_type::AssetMainType;

pub mod contract_type;
pub use contract_type::ContractType;

pub mod div_freq;
pub use div_freq::DivFreq;

pub mod equity_asset_sub_type;
pub use equity_asset_sub_type::EquityAssetSubType;

pub mod exercise_type;
pub use exercise_type::ExerciseType;

pub mod expiration_type;
pub use expiration_type::ExpirationType;

pub mod fund_strategy;
pub use fund_strategy::FundStrategy;

pub mod mutual_fund_asset_sub_type;
pub use mutual_fund_asset_sub_type::MutualFundAssetSubType;

pub mod quote_type;
pub use quote_type::QuoteType;

pub mod settlement_type;
pub use settlement_type::SettlementType;

pub mod error;
pub use error::Error;

pub mod error_source;
pub use error_source::ErrorSource;

// ============================================================================
// Parameter Structs - For API call parameters
// ============================================================================

pub mod get_chain_params;
pub use get_chain_params::GetChainParams;

pub mod get_expiration_chain_params;
pub use get_expiration_chain_params::GetExpirationChainParams;

pub mod get_instrument_by_cusip_params;
pub use get_instrument_by_cusip_params::GetInstrumentByCusipParams;

pub mod get_instruments_params;
pub use get_instruments_params::GetInstrumentsParams;

pub mod get_market_hour_params;
pub use get_market_hour_params::GetMarketHourParams;

pub mod get_market_hours_params;
pub use get_market_hours_params::GetMarketHoursParams;

pub mod get_movers_params;
pub use get_movers_params::GetMoversParams;

pub mod get_price_history_params;
pub use get_price_history_params::GetPriceHistoryParams;

pub mod get_quote_params;
pub use get_quote_params::GetQuoteParams;

pub mod get_quotes_params;
pub use get_quotes_params::GetQuotesParams;
