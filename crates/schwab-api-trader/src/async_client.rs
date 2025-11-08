use schwab_api_core::{ApiClient, AsyncHttpClient, HttpError};
use schwab_api_types::trader::*;
use std::ops::Deref;

use crate::{TraderConfig, TraderParams};

/// Async client for Schwab Trader API
pub struct AsyncTraderClient<C: AsyncHttpClient> {
    client: ApiClient<C, TraderConfig>,
}

impl<C: AsyncHttpClient> AsyncTraderClient<C> {
    pub fn new(client: C) -> Self {
        Self {
            client: ApiClient::new(client),
        }
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
    pub async fn get_account_numbers(
        &self,
        access_token: &str,
    ) -> Result<Vec<AccountNumberHash>, HttpError> {
        let params = TraderParams::get_account_numbers(access_token);
        self.client.fetch(&params).await
    }

    /// Fetch all accounts for the user.
    pub async fn get_accounts(
        &self,
        access_token: &str,
        fields: Option<&str>,
    ) -> Result<Vec<Account>, HttpError> {
        let params = TraderParams::get_accounts(access_token, fields);
        self.client.fetch(&params).await
    }

    /// Fetch a specific account by `account_hash` (Schwab's encrypted account ID).
    pub async fn get_account(
        &self,
        access_token: &str,
        account_hash: &str,
        fields: Option<&str>,
    ) -> Result<Account, HttpError> {
        let params = TraderParams::get_account(access_token, account_hash, fields);
        self.client.fetch(&params).await
    }

    /// Fetch all orders for a specific account.
    pub async fn get_orders_by_path_param(
        &self,
        access_token: &str,
        account_hash: &str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i32>,
        status: Option<&str>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderParams::get_orders_by_path_param(
            access_token,
            account_hash,
            from_entered_time,
            to_entered_time,
            max_results,
            status,
        );
        self.client.fetch(&params).await
    }

    /// Fetch orders across all accounts.
    pub async fn get_orders_by_query_param(
        &self,
        access_token: &str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i32>,
        status: Option<&str>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderParams::get_orders_by_query_param(
            access_token,
            from_entered_time,
            to_entered_time,
            max_results,
            status,
        );
        self.client.fetch(&params).await
    }

    /// Fetch a specific order by its `order_id` for a given account.
    pub async fn get_order(
        &self,
        access_token: &str,
        account_hash: &str,
        order_id: i64,
    ) -> Result<Order, HttpError> {
        let params = TraderParams::get_order(access_token, account_hash, order_id);
        self.client.fetch(&params).await
    }

    /// Place an order for a specific account.
    pub async fn place_order(
        &self,
        access_token: &str,
        account_hash: &str,
        order: &OrderRequest,
    ) -> Result<(), HttpError> {
        let params = TraderParams::place_order(access_token, account_hash, order);
        self.client.execute(&params).await
    }

    /// Replace an existing order.
    pub async fn replace_order(
        &self,
        access_token: &str,
        account_hash: &str,
        order_id: i64,
        order: &OrderRequest,
    ) -> Result<(), HttpError> {
        let params = TraderParams::replace_order(access_token, account_hash, order_id, order);
        self.client.execute(&params).await
    }

    /// Cancel an order.
    pub async fn cancel_order(
        &self,
        access_token: &str,
        account_hash: &str,
        order_id: i64,
    ) -> Result<(), HttpError> {
        let params = TraderParams::cancel_order(access_token, account_hash, order_id);
        self.client.execute(&params).await
    }

    /// Preview an order (dry-run validation).
    pub async fn preview_order(
        &self,
        access_token: &str,
        account_hash: &str,
        order: &PreviewOrder,
    ) -> Result<PreviewOrder, HttpError> {
        let params = TraderParams::preview_order(access_token, account_hash, order);
        self.client.fetch(&params).await
    }

    /// Fetch transactions for a specific account.
    pub async fn get_transactions_by_path_param(
        &self,
        access_token: &str,
        account_hash: &str,
        start_date: &str,
        end_date: &str,
        types: &str,
        symbol: Option<&str>,
    ) -> Result<Vec<Transaction>, HttpError> {
        let params = TraderParams::get_transactions_by_path_param(
            access_token,
            account_hash,
            start_date,
            end_date,
            types,
            symbol,
        );
        self.client.fetch(&params).await
    }

    /// Fetch a specific transaction by its `transaction_id`.
    pub async fn get_transactions_by_id(
        &self,
        access_token: &str,
        account_hash: &str,
        transaction_id: i64,
    ) -> Result<Vec<Transaction>, HttpError> {
        let params =
            TraderParams::get_transactions_by_id(access_token, account_hash, transaction_id);
        self.client.fetch(&params).await
    }

    /// Fetch user preferences for the logged-in user.
    pub async fn get_user_preference(
        &self,
        access_token: &str,
    ) -> Result<UserPreference, HttpError> {
        let params = TraderParams::get_user_preference(access_token);
        self.client.fetch(&params).await
    }
}
