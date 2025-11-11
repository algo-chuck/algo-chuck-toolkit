//! Synchronous client implementation for Schwab Trader API.
//!
//! This module provides a blocking/sync client for interacting with the Schwab Trader API,
//! supporting operations like account management, order placement, and transaction history.

use schwab_api_core::{ApiClient, HttpError, SyncHttpClient};
use schwab_api_types::trader::*;
use std::ops::Deref;

use crate::{TraderConfig, TraderParams};

/// Synchronous/blocking client for Schwab Trader API.
///
/// This client provides blocking methods for all Trader API operations, including:
/// - Account queries and management
/// - Order placement, modification, and cancellation
/// - Transaction history
/// - User preferences
///
/// # Examples
///
/// ```ignore
/// use schwab_api_trader::SyncTraderClient;
///
/// let http_client = ureq::Agent::new();
/// let client = SyncTraderClient::new(http_client);
///
/// let accounts = client.get_account_numbers("your_token")?;
/// ```
pub struct SyncTraderClient<C: SyncHttpClient> {
    client: ApiClient<C, TraderConfig>,
}

impl<C: SyncHttpClient> SyncTraderClient<C> {
    pub fn new(client: C, access_token: impl Into<String>) -> Self {
        Self {
            client: ApiClient::new(client, access_token),
        }
    }
}

impl<C: SyncHttpClient> Deref for SyncTraderClient<C> {
    type Target = ApiClient<C, TraderConfig>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl<C> SyncTraderClient<C>
where
    C: SyncHttpClient,
    HttpError: From<C::Error>,
{
    /// Fetch all account numbers linked to the user.
    pub fn get_account_numbers(&self) -> Result<Vec<AccountNumberHash>, HttpError> {
        let params = TraderParams::get_account_numbers();
        self.client.fetch_sync(&params)
    }

    /// Fetch all accounts for the user.
    pub fn get_accounts(
        &self,
        params: &schwab_api_types::trader_params::GetAccountsParams<'_>,
    ) -> Result<Vec<Account>, HttpError> {
        let params = TraderParams::get_accounts(params);
        self.client.fetch_sync(&params)
    }

    /// Fetch a specific account by `account_hash` (Schwab's encrypted account ID).
    pub fn get_account(
        &self,
        params: &schwab_api_types::trader_params::GetAccountParams<'_>,
    ) -> Result<Account, HttpError> {
        let params = TraderParams::get_account(params);
        self.client.fetch_sync(&params)
    }

    /// Fetch all orders for a specific account.
    pub fn get_orders_by_path_param(
        &self,
        params: &schwab_api_types::trader_params::GetOrdersByPathParams<'_>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderParams::get_orders_by_path_param(params);
        self.client.fetch_sync(&params)
    }

    /// Fetch orders across all accounts.
    pub fn get_orders_by_query_param(
        &self,
        params: &schwab_api_types::trader_params::GetOrdersByQueryParams<'_>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderParams::get_orders_by_query_param(params);
        self.client.fetch_sync(&params)
    }

    /// Fetch a specific order by its `order_id` for a given account.
    pub fn get_order(
        &self,
        params: &schwab_api_types::trader_params::GetOrderParams<'_>,
    ) -> Result<Order, HttpError> {
        let params = TraderParams::get_order(params);
        self.client.fetch_sync(&params)
    }

    /// Place an order for a specific account.
    pub fn place_order(
        &self,
        params: &schwab_api_types::trader_params::PlaceOrderParams<'_>,
    ) -> Result<(), HttpError> {
        let params = TraderParams::place_order(params);
        self.client.execute_sync(&params)
    }

    /// Replace an existing order.
    pub fn replace_order(
        &self,
        params: &schwab_api_types::trader_params::ReplaceOrderParams<'_>,
    ) -> Result<(), HttpError> {
        let params = TraderParams::replace_order(params);
        self.client.execute_sync(&params)
    }

    /// Cancel an order.
    pub fn cancel_order(
        &self,
        params: &schwab_api_types::trader_params::CancelOrderParams<'_>,
    ) -> Result<(), HttpError> {
        let params = TraderParams::cancel_order(params);
        self.client.execute_sync(&params)
    }

    /// Preview an order (dry-run validation).
    pub fn preview_order(
        &self,
        params: &schwab_api_types::trader_params::PreviewOrderParams<'_>,
    ) -> Result<PreviewOrder, HttpError> {
        let params = TraderParams::preview_order(params);
        self.client.fetch_sync(&params)
    }

    /// Fetch transactions for a specific account.
    pub fn get_transactions_by_path_param(
        &self,
        params: &schwab_api_types::trader_params::GetTransactionsByPathParams<'_>,
    ) -> Result<Vec<Transaction>, HttpError> {
        let params = TraderParams::get_transactions_by_path_param(params);
        self.client.fetch_sync(&params)
    }

    /// Fetch a specific transaction by its `transaction_id`.
    pub fn get_transactions_by_id(
        &self,
        params: &schwab_api_types::trader_params::GetTransactionByIdParams<'_>,
    ) -> Result<Vec<Transaction>, HttpError> {
        let params = TraderParams::get_transactions_by_id(params);
        self.client.fetch_sync(&params)
    }

    /// Fetch user preferences for the logged-in user.
    pub fn get_user_preference(&self) -> Result<UserPreference, HttpError> {
        let params = TraderParams::get_user_preference();
        self.client.fetch_sync(&params)
    }
}
