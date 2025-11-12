//! Asynchronous client implementation for Schwab Trader API.
//!
//! This module provides an async client for interacting with the Schwab Trader API,
//! supporting operations like account management, order placement, and transaction history.

use schwab_api_core::{ApiClient, AsyncHttpClient, HttpError};
use schwab_api_types::trader::*;
use std::ops::Deref;

use crate::{TraderConfig, TraderParams};

/// Asynchronous client for Schwab Trader API.
///
/// This client provides async methods for all Trader API operations, including:
/// - Account queries and management
/// - Order placement, modification, and cancellation
/// - Transaction history
/// - User preferences
///
/// The access token is stored internally and can be updated when refreshed.
///
/// # Examples
///
/// ```ignore
/// use schwab_api_trader::AsyncTraderClient;
///
/// let http_client = reqwest::Client::new();
/// let client = AsyncTraderClient::new(http_client, "your_access_token");
///
/// // Use the client
/// let accounts = client.get_account_numbers().await?;
///
/// // Update token when refreshed
/// client.set_access_token("new_access_token");
/// ```
pub struct AsyncTraderClient<C: AsyncHttpClient> {
    client: ApiClient<C, TraderConfig>,
}

impl<C: AsyncHttpClient> AsyncTraderClient<C> {
    /// Create a new async trader client with an access token.
    ///
    /// # Arguments
    ///
    /// * `http_client` - The HTTP client to use for requests
    /// * `access_token` - The OAuth2 access token
    pub fn new(http_client: C, access_token: impl Into<String>) -> Self {
        Self {
            client: ApiClient::new(http_client, access_token),
        }
    }

    /// Update the access token used for authentication.
    ///
    /// This is useful when the token has been refreshed.
    ///
    /// # Arguments
    ///
    /// * `new_token` - The new access token
    pub fn set_access_token(&self, new_token: impl Into<String>) {
        self.client.set_access_token(new_token);
    }

    /// Get a copy of the current access token.
    pub fn get_access_token(&self) -> String {
        self.client.get_access_token()
    }
}

impl<C: AsyncHttpClient> Deref for AsyncTraderClient<C> {
    type Target = ApiClient<C, TraderConfig>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl<C> AsyncTraderClient<C>
where
    C: AsyncHttpClient,
    HttpError: From<C::Error>,
{
    /// Fetch all account numbers linked to the user.
    pub async fn get_account_numbers(&self) -> Result<Vec<AccountNumberHash>, HttpError> {
        let params = TraderParams::get_account_numbers();
        self.client.fetch(&params).await
    }

    /// Fetch all accounts for the user.
    pub async fn get_accounts(
        &self,
        params: &GetAccountsParams<'_>,
    ) -> Result<Vec<Account>, HttpError> {
        let params = TraderParams::get_accounts(params);
        self.client.fetch(&params).await
    }

    /// Fetch a specific account by `account_hash` (Schwab's encrypted account ID).
    pub async fn get_account(&self, params: &GetAccountParams<'_>) -> Result<Account, HttpError> {
        let params = TraderParams::get_account(params);
        self.client.fetch(&params).await
    }

    /// Fetch all orders for a specific account.
    pub async fn get_orders_by_path_param(
        &self,
        params: &GetOrdersByPathParams<'_>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderParams::get_orders_by_path_param(params);
        self.client.fetch(&params).await
    }

    /// Fetch orders across all accounts.
    pub async fn get_orders_by_query_param(
        &self,
        params: &GetOrdersByQueryParams<'_>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderParams::get_orders_by_query_param(params);
        self.client.fetch(&params).await
    }

    /// Fetch a specific order by its `order_id` for a given account.
    pub async fn get_order(&self, params: &GetOrderParams<'_>) -> Result<Order, HttpError> {
        let params = TraderParams::get_order(params);
        self.client.fetch(&params).await
    }

    /// Place an order for a specific account.
    pub async fn place_order(&self, params: &PlaceOrderParams<'_>) -> Result<(), HttpError> {
        let params = TraderParams::place_order(params);
        self.client.execute(&params).await
    }

    /// Replace an existing order.
    pub async fn replace_order(&self, params: &ReplaceOrderParams<'_>) -> Result<(), HttpError> {
        let params = TraderParams::replace_order(params);
        self.client.execute(&params).await
    }

    /// Cancel an order.
    pub async fn cancel_order(&self, params: &CancelOrderParams<'_>) -> Result<(), HttpError> {
        let params = TraderParams::cancel_order(params);
        self.client.execute(&params).await
    }

    /// Preview an order (dry-run validation).
    pub async fn preview_order(
        &self,
        params: &PreviewOrderParams<'_>,
    ) -> Result<PreviewOrder, HttpError> {
        let params = TraderParams::preview_order(params);
        self.client.fetch(&params).await
    }

    /// Fetch transactions for a specific account.
    pub async fn get_transactions_by_path_param(
        &self,
        params: &GetTransactionsByPathParams<'_>,
    ) -> Result<Vec<Transaction>, HttpError> {
        let params = TraderParams::get_transactions_by_path_param(params);
        self.client.fetch(&params).await
    }

    /// Fetch a specific transaction by its `transaction_id`.
    pub async fn get_transactions_by_id(
        &self,
        params: &GetTransactionByIdParams<'_>,
    ) -> Result<Vec<Transaction>, HttpError> {
        let params = TraderParams::get_transactions_by_id(params);
        self.client.fetch(&params).await
    }

    /// Fetch user preferences for the logged-in user.
    pub async fn get_user_preference(&self) -> Result<UserPreference, HttpError> {
        let params = TraderParams::get_user_preference();
        self.client.fetch(&params).await
    }
}
