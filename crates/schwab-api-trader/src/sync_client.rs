use schwab_api_core::{ApiClient, HttpError, RequestParams, SyncHttpClient};
use schwab_api_types::trader::*;
use std::ops::Deref;

use crate::TraderConfig;

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
        let params = RequestParams {
            access_token,
            method: http::Method::GET,
            path: "/accounts/accountNumbers".to_string(),
            query: None,
            body: None::<()>,
        };
        self.client.fetch_sync(&params)
    }

    /// Fetch all accounts for the user.
    pub fn get_accounts(
        &self,
        access_token: &str,
        fields: Option<&str>,
    ) -> Result<Vec<Account>, HttpError> {
        let query_string = fields.map(|f| format!("fields={}", f));
        let params = RequestParams {
            access_token,
            method: http::Method::GET,
            path: "/accounts".to_string(),
            query: query_string,
            body: None::<()>,
        };
        self.client.fetch_sync(&params)
    }

    /// Fetch a specific account by `encrypted_id` (Schwab's encrypted account ID).
    pub fn get_account(
        &self,
        access_token: &str,
        encrypted_id: &str,
        fields: Option<&str>,
    ) -> Result<Account, HttpError> {
        let path = format!("/accounts/{}", encrypted_id);
        let query_string = fields.map(|f| format!("fields={}", f));
        let params = RequestParams {
            access_token,
            method: http::Method::GET,
            path,
            query: query_string,
            body: None::<()>,
        };
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
        let path = format!("/accounts/{}/orders", encrypted_id);
        let mut query_parts = vec![
            format!("fromEnteredTime={}", from_entered_time),
            format!("toEnteredTime={}", to_entered_time),
        ];

        if let Some(max) = max_results {
            query_parts.push(format!("maxResults={}", max));
        }
        if let Some(status_value) = status {
            query_parts.push(format!("status={}", status_value));
        }

        let params = RequestParams {
            access_token,
            method: http::Method::GET,
            path,
            query: Some(query_parts.join("&")),
            body: None::<()>,
        };
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
        let mut query_parts = vec![
            format!("fromEnteredTime={}", from_entered_time),
            format!("toEnteredTime={}", to_entered_time),
        ];

        if let Some(max) = max_results {
            query_parts.push(format!("maxResults={}", max));
        }
        if let Some(status_value) = status {
            query_parts.push(format!("status={}", status_value));
        }

        let params = RequestParams {
            access_token,
            method: http::Method::GET,
            path: "/orders".to_string(),
            query: Some(query_parts.join("&")),
            body: None::<()>,
        };
        self.client.fetch_sync(&params)
    }

    /// Fetch a specific order by its `order_id` for a given account.
    pub fn get_order(
        &self,
        access_token: &str,
        encrypted_id: &str,
        order_id: i64,
    ) -> Result<Order, HttpError> {
        let path = format!("/accounts/{}/orders/{}", encrypted_id, order_id);

        let params = RequestParams {
            access_token,
            method: http::Method::GET,
            path,
            query: None,
            body: None::<()>,
        };
        self.client.fetch_sync(&params)
    }

    /// Place an order for a specific account.
    pub fn place_order(
        &self,
        access_token: &str,
        encrypted_id: &str,
        order: &OrderRequest,
    ) -> Result<(), HttpError> {
        let path = format!("/accounts/{}/orders", encrypted_id);

        let params = RequestParams {
            access_token,
            method: http::Method::POST,
            path,
            query: None,
            body: Some(order),
        };
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
        let path = format!("/accounts/{}/orders/{}", encrypted_id, order_id);

        let params = RequestParams {
            access_token,
            method: http::Method::PUT,
            path,
            query: None,
            body: Some(order),
        };
        self.client.execute_sync(&params)
    }

    /// Cancel an order.
    pub fn cancel_order(
        &self,
        access_token: &str,
        encrypted_id: &str,
        order_id: i64,
    ) -> Result<(), HttpError> {
        let path = format!("/accounts/{}/orders/{}", encrypted_id, order_id);

        let params = RequestParams {
            access_token,
            method: http::Method::DELETE,
            path,
            query: None,
            body: None::<()>,
        };
        self.client.execute_sync(&params)
    }

    /// Preview an order (dry-run validation).
    pub fn preview_order(
        &self,
        access_token: &str,
        encrypted_id: &str,
        order: &OrderRequest,
    ) -> Result<PreviewOrder, HttpError> {
        let path = format!("/accounts/{}/previewOrder", encrypted_id);

        let params = RequestParams {
            access_token,
            method: http::Method::POST,
            path,
            query: None,
            body: Some(order),
        };
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
        let path = format!("/accounts/{}/transactions", encrypted_id);

        let mut query_parts = vec![
            format!("startDate={}", start_date),
            format!("endDate={}", end_date),
            format!("types={}", types),
        ];

        if let Some(sym) = symbol {
            query_parts.push(format!("symbol={}", sym));
        }

        let params = RequestParams {
            access_token,
            method: http::Method::GET,
            path,
            query: Some(query_parts.join("&")),
            body: None::<()>,
        };
        self.client.fetch_sync(&params)
    }

    /// Fetch a specific transaction by its `transaction_id`.
    pub fn get_transactions_by_id(
        &self,
        access_token: &str,
        encrypted_id: &str,
        transaction_id: i64,
    ) -> Result<Vec<Transaction>, HttpError> {
        let path = format!("/accounts/{}/transactions/{}", encrypted_id, transaction_id);

        let params = RequestParams {
            access_token,
            method: http::Method::GET,
            path,
            query: None,
            body: None::<()>,
        };
        self.client.fetch_sync(&params)
    }

    /// Fetch user preferences for the logged-in user.
    pub fn get_user_preference(&self, access_token: &str) -> Result<UserPreference, HttpError> {
        let params = RequestParams {
            access_token,
            method: http::Method::GET,
            path: "/userPreference".to_string(),
            query: None,
            body: None::<()>,
        };
        self.client.fetch_sync(&params)
    }
}
