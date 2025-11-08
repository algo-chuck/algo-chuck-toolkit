use schwab_api_core::{ApiClient, HttpError, SyncHttpClient};
use schwab_api_types::trader::*;
use std::ops::Deref;

use crate::{TraderConfig, TraderParams, TraderParamsImpl};

/// Synchronous/blocking client for Schwab Trader API
pub struct SyncTraderClient<C: SyncHttpClient> {
    client: ApiClient<C, TraderConfig>,
}

impl<C: SyncHttpClient> SyncTraderClient<C> {
    pub fn new(client: C) -> Self {
        Self {
            client: ApiClient::new(client),
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
        access_token: &str,
    ) -> Result<Vec<AccountNumberHash>, HttpError> {
        let params = TraderParamsImpl::get_account_numbers(access_token);
        self.client.fetch_sync(&params)
    }

    /// Fetch all accounts for the user.
    pub fn get_accounts(
        &self,
        access_token: &str,
        fields: Option<&str>,
    ) -> Result<Vec<Account>, HttpError> {
        let params = TraderParamsImpl::get_accounts(access_token, fields);
        self.client.fetch_sync(&params)
    }

    /// Fetch a specific account by `encrypted_id` (Schwab's encrypted account ID).
    pub fn get_account(
        &self,
        access_token: &str,
        encrypted_id: &str,
        fields: Option<&str>,
    ) -> Result<Account, HttpError> {
        let params = TraderParamsImpl::get_account(access_token, encrypted_id, fields);
        self.client.fetch_sync(&params)
    }

    /// Fetch all orders for a specific account.
    pub fn get_orders_by_path_param(
        &self,
        access_token: &str,
        encrypted_id: &str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i32>,
        status: Option<&str>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderParamsImpl::get_orders_by_path_param(
            access_token,
            encrypted_id,
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
        access_token: &str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i32>,
        status: Option<&str>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderParamsImpl::get_orders_by_query_param(
            access_token,
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
        access_token: &str,
        encrypted_id: &str,
        order_id: i64,
    ) -> Result<Order, HttpError> {
        let params = TraderParamsImpl::get_order(access_token, encrypted_id, order_id);
        self.client.fetch_sync(&params)
    }

    /// Place an order for a specific account.
    pub fn place_order(
        &self,
        access_token: &str,
        encrypted_id: &str,
        order: &OrderRequest,
    ) -> Result<(), HttpError> {
        let params = TraderParamsImpl::place_order(access_token, encrypted_id, order);
        self.client.execute_sync(&params)
    }

    /// Replace an existing order.
    pub fn replace_order(
        &self,
        access_token: &str,
        encrypted_id: &str,
        order_id: i64,
        order: &OrderRequest,
    ) -> Result<(), HttpError> {
        let params = TraderParamsImpl::replace_order(access_token, encrypted_id, order_id, order);
        self.client.execute_sync(&params)
    }

    /// Cancel an order.
    pub fn cancel_order(
        &self,
        access_token: &str,
        encrypted_id: &str,
        order_id: i64,
    ) -> Result<(), HttpError> {
        let params = TraderParamsImpl::cancel_order(access_token, encrypted_id, order_id);
        self.client.execute_sync(&params)
    }

    /// Preview an order (dry-run validation).
    pub fn preview_order(
        &self,
        access_token: &str,
        encrypted_id: &str,
        order: &OrderRequest,
    ) -> Result<PreviewOrder, HttpError> {
        let params = TraderParamsImpl::preview_order(access_token, encrypted_id, order);
        self.client.fetch_sync(&params)
    }

    /// Fetch transactions for a specific account.
    pub fn get_transactions_by_path_param(
        &self,
        access_token: &str,
        encrypted_id: &str,
        start_date: &str,
        end_date: &str,
        types: &str,
        symbol: Option<&str>,
    ) -> Result<Vec<Transaction>, HttpError> {
        let params = TraderParamsImpl::get_transactions_by_path_param(
            access_token,
            encrypted_id,
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
        access_token: &str,
        encrypted_id: &str,
        transaction_id: i64,
    ) -> Result<Vec<Transaction>, HttpError> {
        let params =
            TraderParamsImpl::get_transactions_by_id(access_token, encrypted_id, transaction_id);
        self.client.fetch_sync(&params)
    }

    /// Fetch user preferences for the logged-in user.
    pub fn get_user_preference(&self, access_token: &str) -> Result<UserPreference, HttpError> {
        let params = TraderParamsImpl::get_user_preference(access_token);
        self.client.fetch_sync(&params)
    }
}
