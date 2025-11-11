//! Request parameter builders for Schwab Trader API endpoints.
//!
//! This module provides type-safe parameter construction for all Trader API operations.
//! Each method corresponds to an API endpoint and returns a `RequestParams` struct
//! configured with the appropriate HTTP method, path, and query parameters.

use http::Method;

use schwab_api_core::RequestParams;
use schwab_api_types::{OrderRequest, PreviewOrder};

/// Parameter builders for all Schwab Trader API endpoints.
///
/// Function names match OpenAPI operationIds (converted to snake_case).
/// All methods are static and return `RequestParams` configured for the specific endpoint.
pub struct TraderParams;

impl TraderParams {
    /// Build params for getAccountNumbers operation
    pub fn get_account_numbers<'a>(access_token: &'a str) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: "/accounts/accountNumbers".to_string(),
            method: Method::GET,
            query: None,
        }
    }

    /// Build params for getAccounts operation
    pub fn get_accounts<'a>(access_token: &'a str, fields: Option<&str>) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: "/accounts".to_string(),
            method: Method::GET,
            query: fields.map(|f| format!("fields={f}")),
        }
    }

    /// Build params for getAccount operation
    pub fn get_account<'a>(
        access_token: &'a str,
        account_hash: &str,
        fields: Option<&str>,
    ) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: format!("/accounts/{account_hash}"),
            method: Method::GET,
            query: fields.map(|f| format!("fields={f}")),
        }
    }

    /// Build params for getOrdersByPathParam operation
    pub fn get_orders_by_path_param<'a>(
        access_token: &'a str,
        account_hash: &str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i32>,
        status: Option<&str>,
    ) -> RequestParams<'a> {
        let mut query_parts = vec![
            format!("fromEnteredTime={from_entered_time}"),
            format!("toEnteredTime={to_entered_time}"),
        ];
        if let Some(max) = max_results {
            query_parts.push(format!("maxResults={max}"));
        }
        if let Some(s) = status {
            query_parts.push(format!("status={s}"));
        }
        RequestParams {
            access_token,
            body: None,
            path: format!("/accounts/{account_hash}/orders"),
            method: Method::GET,
            query: Some(query_parts.join("&")),
        }
    }

    /// Build params for getOrder operation
    pub fn get_order<'a>(
        access_token: &'a str,
        account_hash: &str,
        order_id: i64,
    ) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: format!("/accounts/{account_hash}/orders/{order_id}"),
            method: Method::GET,
            query: None,
        }
    }

    /// Build params for placeOrder operation
    pub fn place_order<'a>(
        access_token: &'a str,
        account_hash: &str,
        order: &'a OrderRequest,
    ) -> RequestParams<'a, &'a OrderRequest> {
        RequestParams {
            access_token,
            body: Some(order),
            path: format!("/accounts/{account_hash}/orders"),
            method: Method::POST,
            query: None,
        }
    }

    /// Build params for cancelOrder operation
    pub fn cancel_order<'a>(
        access_token: &'a str,
        account_hash: &str,
        order_id: i64,
    ) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: format!("/accounts/{account_hash}/orders/{order_id}"),
            method: Method::DELETE,
            query: None,
        }
    }

    /// Build params for replaceOrder operation
    pub fn replace_order<'a>(
        access_token: &'a str,
        account_hash: &str,
        order_id: i64,
        order: &'a OrderRequest,
    ) -> RequestParams<'a, &'a OrderRequest> {
        RequestParams {
            access_token,
            body: Some(order),
            path: format!("/accounts/{account_hash}/orders/{order_id}"),
            method: Method::PUT,
            query: None,
        }
    }

    /// Build params for getOrdersByQueryParam operation
    pub fn get_orders_by_query_param<'a>(
        access_token: &'a str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i32>,
        status: Option<&str>,
    ) -> RequestParams<'a> {
        let mut query_parts = vec![
            format!("fromEnteredTime={from_entered_time}"),
            format!("toEnteredTime={to_entered_time}"),
        ];
        if let Some(max) = max_results {
            query_parts.push(format!("maxResults={max}"));
        }
        if let Some(s) = status {
            query_parts.push(format!("status={s}"));
        }
        RequestParams {
            access_token,
            body: None,
            path: "/orders".to_string(),
            method: Method::GET,
            query: Some(query_parts.join("&")),
        }
    }

    /// Build params for previewOrder operation
    pub fn preview_order<'a>(
        access_token: &'a str,
        account_hash: &str,
        order: &'a PreviewOrder,
    ) -> RequestParams<'a, &'a PreviewOrder> {
        RequestParams {
            access_token,
            body: Some(order),
            path: format!("/accounts/{account_hash}/previewOrder"),
            method: Method::POST,
            query: None,
        }
    }

    /// Build params for getTransactionsByPathParam operation
    pub fn get_transactions_by_path_param<'a>(
        access_token: &'a str,
        account_hash: &str,
        start_date: &str,
        end_date: &str,
        types: &str,
        symbol: Option<&str>,
    ) -> RequestParams<'a> {
        let mut query_parts = vec![
            format!("startDate={start_date}"),
            format!("endDate={end_date}"),
            format!("types={types}"),
        ];
        if let Some(sym) = symbol {
            query_parts.push(format!("symbol={sym}"));
        }
        RequestParams {
            access_token,
            body: None,
            path: format!("/accounts/{account_hash}/transactions"),
            method: Method::GET,
            query: Some(query_parts.join("&")),
        }
    }

    /// Build params for getTransactionsById operation
    pub fn get_transactions_by_id<'a>(
        access_token: &'a str,
        account_hash: &str,
        transaction_id: i64,
    ) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: format!("/accounts/{account_hash}/transactions/{transaction_id}"),
            method: Method::GET,
            query: None,
        }
    }

    /// Build params for getUserPreference operation
    pub fn get_user_preference<'a>(access_token: &'a str) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: "/userPreference".to_string(),
            method: Method::GET,
            query: None,
        }
    }
}
