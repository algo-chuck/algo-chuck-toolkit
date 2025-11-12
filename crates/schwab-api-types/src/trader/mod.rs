//! Trader API types for accounts, orders, and transactions.
//!
//! This module contains all types used in the Schwab Trader API including:
//! - Account information and balances
//! - Order placement and management
//! - Transaction history
//! - Position details
//!
//! Types are organized into response types (received from API), request types (sent to API),
//! enums (shared types), and parameter structs (for API calls).

// ============================================================================
// Response Types - Returned by API operations
// ============================================================================

pub mod account;
pub use account::Account;

pub mod cash_account;
pub use cash_account::CashAccount;

pub mod cash_balance;
pub use cash_balance::CashBalance;

pub mod cash_initial_balance;
pub use cash_initial_balance::CashInitialBalance;

pub mod commission;
pub use commission::Commission;

pub mod commission_and_fee;
pub use commission_and_fee::CommissionAndFee;

pub mod commission_leg;
pub use commission_leg::CommissionLeg;

pub mod commission_value;
pub use commission_value::CommissionValue;

pub mod date_param;
pub use date_param::DateParam;

pub mod execution_leg;
pub use execution_leg::ExecutionLeg;

pub mod fee_leg;
pub use fee_leg::FeeLeg;

pub mod fee_value;
pub use fee_value::FeeValue;

pub mod fees;
pub use fees::Fees;

pub mod margin_account;
pub use margin_account::MarginAccount;

pub mod margin_balance;
pub use margin_balance::MarginBalance;

pub mod margin_initial_balance;
pub use margin_initial_balance::MarginInitialBalance;

pub mod offer;
pub use offer::Offer;

pub mod order;
pub use order::Order;

pub mod order_activity;
pub use order_activity::OrderActivity;

pub mod order_balance;
pub use order_balance::OrderBalance;

pub mod order_leg;
pub use order_leg::OrderLeg;

pub mod order_leg_collection;
pub use order_leg_collection::OrderLegCollection;

pub mod order_strategy;
pub use order_strategy::OrderStrategy;

pub mod order_validation_detail;
pub use order_validation_detail::OrderValidationDetail;

pub mod order_validation_result;
pub use order_validation_result::OrderValidationResult;

pub mod position;
pub use position::Position;

pub mod preview_order;
pub use preview_order::PreviewOrder;

pub mod securities_account;
pub use securities_account::SecuritiesAccount;

pub mod securities_account_base;
pub use securities_account_base::SecuritiesAccountBase;

pub mod service_error;
pub use service_error::ServiceError;

pub mod streamer_info;
pub use streamer_info::StreamerInfo;

pub mod transaction;
pub use transaction::Transaction;

pub mod transfer_item;

pub mod user_details;

pub mod user_preference;
pub use user_preference::UserPreference;

pub mod user_preference_account;
pub use user_preference_account::UserPreferenceAccount;

// ============================================================================
// Request Types - Sent to API operations
// ============================================================================

pub mod order_request;
pub use order_request::OrderRequest;

// ============================================================================
// Instrument Types
// ============================================================================

pub mod account_api_option_deliverable;
pub use account_api_option_deliverable::AccountApiOptionDeliverable;

pub mod account_cash_equivalent;
pub use account_cash_equivalent::AccountCashEquivalent;

pub mod account_equity;
pub use account_equity::AccountEquity;

pub mod account_fixed_income;
pub use account_fixed_income::AccountFixedIncome;

pub mod account_mutual_fund;
pub use account_mutual_fund::AccountMutualFund;

pub mod account_number_hash;
pub use account_number_hash::AccountNumberHash;

pub mod account_option;
pub use account_option::AccountOption;

pub mod accounts_base_instrument;
pub use accounts_base_instrument::AccountsBaseInstrument;

pub mod accounts_instrument;
pub use accounts_instrument::AccountsInstrument;

pub mod collective_investment;
pub use collective_investment::CollectiveInvestment;

pub mod currency;
pub use currency::Currency;

pub mod forex;
pub use forex::Forex;

pub mod future;
pub use future::Future;

pub mod index;
pub use index::Index;

pub mod product;
pub use product::Product;

pub mod transaction_api_option_deliverable;
pub use transaction_api_option_deliverable::TransactionApiOptionDeliverable;

pub mod transaction_base_instrument;
pub use transaction_base_instrument::TransactionBaseInstrument;

pub mod transaction_cash_equivalent;
pub use transaction_cash_equivalent::TransactionCashEquivalent;

pub mod transaction_equity;
pub use transaction_equity::TransactionEquity;

pub mod transaction_fixed_income;
pub use transaction_fixed_income::TransactionFixedIncome;

pub mod transaction_instrument;
pub use transaction_instrument::TransactionInstrument;

pub mod transaction_mutual_fund;
pub use transaction_mutual_fund::TransactionMutualFund;

pub mod transaction_option;
pub use transaction_option::TransactionOption;

// ============================================================================
// Enums - Shared types for requests and responses
// ============================================================================

pub mod amount_indicator;
pub use amount_indicator::AmountIndicator;

pub mod api_order_status;
pub use api_order_status::ApiOrderStatus;

pub mod api_rule_action;
pub use api_rule_action::ApiRuleAction;

pub mod asset_type;
pub use asset_type::AssetType;

pub mod complex_order_strategy_type;
pub use complex_order_strategy_type::ComplexOrderStrategyType;

pub mod duration;
pub use duration::Duration;

pub mod fee_type;
pub use fee_type::FeeType;

pub mod instruction;
pub use instruction::Instruction;

pub mod order_strategy_type;
pub use order_strategy_type::OrderStrategyType;

pub mod order_type;
pub use order_type::OrderType;

pub mod order_type_request;
pub use order_type_request::OrderTypeRequest;

pub mod price_link_basis;
pub use price_link_basis::PriceLinkBasis;

pub mod price_link_type;
pub use price_link_type::PriceLinkType;

pub mod requested_destination;
pub use requested_destination::RequestedDestination;

pub mod session;
pub use session::Session;

pub mod settlement_instruction;
pub use settlement_instruction::SettlementInstruction;

pub mod special_instruction;
pub use special_instruction::SpecialInstruction;

pub mod status;
pub use status::Status;

pub mod stop_price_link_basis;
pub use stop_price_link_basis::StopPriceLinkBasis;

pub mod stop_price_link_type;
pub use stop_price_link_type::StopPriceLinkType;

pub mod stop_type;
pub use stop_type::StopType;

pub mod tax_lot_method;
pub use tax_lot_method::TaxLotMethod;

pub mod transaction_type;
pub use transaction_type::TransactionType;

// ============================================================================
// Parameter Structs - For API call parameters
// ============================================================================

pub mod cancel_order_params;
pub use cancel_order_params::CancelOrderParams;

pub mod get_account_params;
pub use get_account_params::GetAccountParams;

pub mod get_accounts_params;
pub use get_accounts_params::GetAccountsParams;

pub mod get_order_params;
pub use get_order_params::GetOrderParams;

pub mod get_orders_by_path_params;
pub use get_orders_by_path_params::GetOrdersByPathParams;

pub mod get_orders_by_query_params;
pub use get_orders_by_query_params::GetOrdersByQueryParams;

pub mod get_transaction_by_id_params;
pub use get_transaction_by_id_params::GetTransactionByIdParams;

pub mod get_transactions_by_path_params;
pub use get_transactions_by_path_params::GetTransactionsByPathParams;

pub mod place_order_params;
pub use place_order_params::PlaceOrderParams;

pub mod preview_order_params;
pub use preview_order_params::PreviewOrderParams;

pub mod replace_order_params;
pub use replace_order_params::ReplaceOrderParams;
