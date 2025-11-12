//! Schwab API type definitions.
//!
//! This crate provides strongly-typed Rust definitions for the Schwab API.
//! Types are organized into two main modules:
//!
//! - [`trader`] - Account, order, and transaction types (requires `trader` feature)
//! - [`marketdata`] - Quote, option chain, and market data types (requires `marketdata` feature)
//!
//! # Features
//!
//! - `trader` - Enables trader API types (enabled by default)
//! - `marketdata` - Enables market data API types (enabled by default)
//! - `default` - Enables both `trader` and `marketdata`
//!
//! # Usage
//!
//! Import types with their module prefix for clarity:
//!
//! ```ignore
//! use schwab_api_types::trader::{Account, Order, OrderRequest};
//! use schwab_api_types::marketdata::{QuoteEquity, OptionChain};
//! ```
//!
//! To use only specific APIs, disable default features:
//!
//! ```toml
//! [dependencies]
//! schwab-api-types = { version = "0.1", default-features = false, features = ["trader"] }
//! ```

#[cfg(feature = "trader")]
pub mod trader;

#[cfg(feature = "marketdata")]
pub mod marketdata;
