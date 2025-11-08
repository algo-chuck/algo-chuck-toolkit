use http::Method;

use schwab_api_core::RequestParams;
use schwab_api_types::OrderRequest;

/// Trait providing parameter builders for all Schwab Trader API endpoints.
/// Function names match OpenAPI operationIds (converted to snake_case).
pub trait TraderParams {
    // Accounts

    /// Build params for getAccountNumbers operation
    fn get_account_numbers<'a>(access_token: &'a str) -> RequestParams<'a>;

    /// Build params for getAccounts operation
    fn get_accounts<'a>(access_token: &'a str, fields: Option<&str>) -> RequestParams<'a>;

    /// Build params for getAccount operation
    fn get_account<'a>(
        access_token: &'a str,
        account_number: &str,
        fields: Option<&str>,
    ) -> RequestParams<'a>;

    // Orders

    /// Build params for getOrdersByPathParam operation
    fn get_orders_by_path_param<'a>(
        access_token: &'a str,
        account_number: &str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i32>,
        status: Option<&str>,
    ) -> RequestParams<'a>;

    /// Build params for getOrder operation
    fn get_order<'a>(
        access_token: &'a str,
        account_number: &str,
        order_id: i64,
    ) -> RequestParams<'a>;

    /// Build params for placeOrder operation
    fn place_order<'a>(
        access_token: &'a str,
        account_number: &str,
        order: &'a OrderRequest,
    ) -> RequestParams<'a, &'a OrderRequest>;

    /// Build params for cancelOrder operation
    fn cancel_order<'a>(
        access_token: &'a str,
        account_number: &str,
        order_id: i64,
    ) -> RequestParams<'a>;

    /// Build params for replaceOrder operation
    fn replace_order<'a>(
        access_token: &'a str,
        account_number: &str,
        order_id: i64,
        order: &'a OrderRequest,
    ) -> RequestParams<'a, &'a OrderRequest>;

    /// Build params for getOrdersByQueryParam operation
    fn get_orders_by_query_param<'a>(
        access_token: &'a str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i32>,
        status: Option<&str>,
    ) -> RequestParams<'a>;

    /// Build params for previewOrder operation
    fn preview_order<'a>(
        access_token: &'a str,
        account_number: &str,
        order: &'a OrderRequest,
    ) -> RequestParams<'a, &'a OrderRequest>;

    // Transactions

    /// Build params for getTransactionsByPathParam operation
    fn get_transactions_by_path_param<'a>(
        access_token: &'a str,
        account_number: &str,
        start_date: &str,
        end_date: &str,
        types: &str,
        symbol: Option<&str>,
    ) -> RequestParams<'a>;

    /// Build params for getTransactionsById operation
    fn get_transactions_by_id<'a>(
        access_token: &'a str,
        account_number: &str,
        transaction_id: i64,
    ) -> RequestParams<'a>;

    // User Preference

    /// Build params for getUserPreference operation
    fn get_user_preference<'a>(access_token: &'a str) -> RequestParams<'a>;
}

/// Concrete implementation of TraderParams
pub struct TraderParamsImpl;

impl TraderParams for TraderParamsImpl {
    fn get_account_numbers<'a>(access_token: &'a str) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: "/accounts/accountNumbers".to_string(),
            method: Method::GET,
            query: None,
        }
    }

    fn get_accounts<'a>(access_token: &'a str, fields: Option<&str>) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: "/accounts".to_string(),
            method: Method::GET,
            query: fields.map(|f| format!("fields={f}")),
        }
    }

    fn get_account<'a>(
        access_token: &'a str,
        account_number: &str,
        fields: Option<&str>,
    ) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: format!("/accounts/{account_number}"),
            method: Method::GET,
            query: fields.map(|f| format!("fields={f}")),
        }
    }

    fn get_orders_by_path_param<'a>(
        access_token: &'a str,
        account_number: &str,
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
            path: format!("/accounts/{account_number}/orders"),
            method: Method::GET,
            query: Some(query_parts.join("&")),
        }
    }

    fn get_order<'a>(
        access_token: &'a str,
        account_number: &str,
        order_id: i64,
    ) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: format!("/accounts/{account_number}/orders/{order_id}"),
            method: Method::GET,
            query: None,
        }
    }

    fn place_order<'a>(
        access_token: &'a str,
        account_number: &str,
        order: &'a OrderRequest,
    ) -> RequestParams<'a, &'a OrderRequest> {
        RequestParams {
            access_token,
            body: Some(order),
            path: format!("/accounts/{account_number}/orders"),
            method: Method::POST,
            query: None,
        }
    }

    fn cancel_order<'a>(
        access_token: &'a str,
        account_number: &str,
        order_id: i64,
    ) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: format!("/accounts/{account_number}/orders/{order_id}"),
            method: Method::DELETE,
            query: None,
        }
    }

    fn replace_order<'a>(
        access_token: &'a str,
        account_number: &str,
        order_id: i64,
        order: &'a OrderRequest,
    ) -> RequestParams<'a, &'a OrderRequest> {
        RequestParams {
            access_token,
            body: Some(order),
            path: format!("/accounts/{account_number}/orders/{order_id}"),
            method: Method::PUT,
            query: None,
        }
    }

    fn get_orders_by_query_param<'a>(
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

    fn preview_order<'a>(
        access_token: &'a str,
        account_number: &str,
        order: &'a OrderRequest,
    ) -> RequestParams<'a, &'a OrderRequest> {
        RequestParams {
            access_token,
            body: Some(order),
            path: format!("/accounts/{account_number}/previewOrder"),
            method: Method::POST,
            query: None,
        }
    }

    fn get_transactions_by_path_param<'a>(
        access_token: &'a str,
        account_number: &str,
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
            path: format!("/accounts/{account_number}/transactions"),
            method: Method::GET,
            query: Some(query_parts.join("&")),
        }
    }

    fn get_transactions_by_id<'a>(
        access_token: &'a str,
        account_number: &str,
        transaction_id: i64,
    ) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: format!("/accounts/{account_number}/transactions/{transaction_id}"),
            method: Method::GET,
            query: None,
        }
    }

    fn get_user_preference<'a>(access_token: &'a str) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: "/userPreference".to_string(),
            method: Method::GET,
            query: None,
        }
    }
}
