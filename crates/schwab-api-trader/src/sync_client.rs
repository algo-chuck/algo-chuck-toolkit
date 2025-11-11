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
    pub fn get_account_numbers(
        &self,
        ) -> Result<Vec<AccountNumberHash>, HttpError> {
        let params = TraderParams::get_account_numbers();
        self.client.fetch_sync(&params)
    }

    /// Fetch all accounts for the user.
    pub fn get_accounts(
        &self,
        fields: Option<&str>,
    ) -> Result<Vec<Account>, HttpError> {
        let params = TraderParams::get_accounts(fields);
        self.client.fetch_sync(&params)
    }

    /// Fetch a specific account by `account_hash` (Schwab's encrypted account ID).
    pub fn get_account(
        &self,
        account_hash: &str,
        fields: Option<&str>,
    ) -> Result<Account, HttpError> {
        let params = TraderParams::get_account(account_hash, fields);
        self.client.fetch_sync(&params)
    }

    /// Fetch all orders for a specific account.
    pub fn get_orders_by_path_param(
        &self,
        account_hash: &str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i32>,
        status: Option<&str>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderParams::get_orders_by_path_param(
            
            account_hash,
            from_entered_time,
            to_entered_time,
            max_results,
            status,
        );
        self.client.fetch_sync(&params)
    }

    /// Fetch orders across all accounts.
    pub fn get_orders_by_query_param(
        &self,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i32>,
        status: Option<&str>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderParams::get_orders_by_query_param(
            
            from_entered_time,
            to_entered_time,
            max_results,
            status,
        );
        self.client.fetch_sync(&params)
    }

    /// Fetch a specific order by its `order_id` for a given account.
    pub fn get_order(
        &self,
        account_hash: &str,
        order_id: i64,
    ) -> Result<Order, HttpError> {
        let params = TraderParams::get_order(account_hash, order_id);
        self.client.fetch_sync(&params)
    }

    /// Place an order for a specific account.
    pub fn place_order(
        &self,
        account_hash: &str,
        order: &OrderRequest,
    ) -> Result<(), HttpError> {
        let params = TraderParams::place_order(account_hash, order);
        self.client.execute_sync(&params)
    }

    /// Replace an existing order.
    pub fn replace_order(
        &self,
        account_hash: &str,
        order_id: i64,
        order: &OrderRequest,
    ) -> Result<(), HttpError> {
        let params = TraderParams::replace_order(account_hash, order_id, order);
        self.client.execute_sync(&params)
    }

    /// Cancel an order.
    pub fn cancel_order(
        &self,
        account_hash: &str,
        order_id: i64,
    ) -> Result<(), HttpError> {
        let params = TraderParams::cancel_order(account_hash, order_id);
        self.client.execute_sync(&params)
    }

    /// Preview an order (dry-run validation).
    pub fn preview_order(
        &self,
        account_hash: &str,
        order: &PreviewOrder,
    ) -> Result<PreviewOrder, HttpError> {
        let params = TraderParams::preview_order(account_hash, order);
        self.client.fetch_sync(&params)
    }

    /// Fetch transactions for a specific account.
    pub fn get_transactions_by_path_param(
        &self,
        account_hash: &str,
        start_date: &str,
        end_date: &str,
        types: &str,
        symbol: Option<&str>,
    ) -> Result<Vec<Transaction>, HttpError> {
        let params = TraderParams::get_transactions_by_path_param(
            
            account_hash,
            start_date,
            end_date,
            types,
            symbol,
        );
        self.client.fetch_sync(&params)
    }

    /// Fetch a specific transaction by its `transaction_id`.
    pub fn get_transactions_by_id(
        &self,
        account_hash: &str,
        transaction_id: i64,
    ) -> Result<Vec<Transaction>, HttpError> {
        let params =
            TraderParams::get_transactions_by_id(account_hash, transaction_id);
        self.client.fetch_sync(&params)
    }

    /// Fetch user preferences for the logged-in user.
    pub fn get_user_preference(&self, ) -> Result<UserPreference, HttpError> {
        let params = TraderParams::get_user_preference();
        self.client.fetch_sync(&params)
    }
}
