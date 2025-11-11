//! Request parameter builders for Schwab Trader API endpoints.
//!
//! This module provides simplified parameter construction using the public parameter
//! structs from `schwab-api-types`. Each method accepts a parameter struct and returns
//! a `RequestParams` configured for the specific endpoint.
//!
//! The access token is stored in the `ApiClient`, so it's not included in these structs.

use http::Method;

use schwab_api_core::RequestParams;
use schwab_api_types::trader_params::*;

/// Parameter builders for all Schwab Trader API endpoints.
///
/// Function names match OpenAPI operationIds (converted to snake_case).
/// All methods are static and return `RequestParams` configured for the specific endpoint.
pub struct TraderParams;

impl TraderParams {
    /// Build params for getAccountNumbers operation
    pub fn get_account_numbers() -> RequestParams {
        RequestParams {
            body: None,
            path: "/accounts/accountNumbers".to_string(),
            method: Method::GET,
            query: None,
        }
    }

    /// Build params for getAccounts operation
    pub fn get_accounts(params: &GetAccountsParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            body: None,
            path: "/accounts".to_string(),
            method: Method::GET,
            query,
        }
    }

    /// Build params for getAccount operation
    pub fn get_account(params: &GetAccountParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            body: None,
            path: format!("/accounts/{}", params.account_hash),
            method: Method::GET,
            query,
        }
    }

    /// Build params for getOrdersByPathParam operation
    pub fn get_orders_by_path_param(params: &GetOrdersByPathParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            body: None,
            path: format!("/accounts/{}/orders", params.account_hash),
            method: Method::GET,
            query,
        }
    }

    /// Build params for getOrder operation
    pub fn get_order(params: &GetOrderParams) -> RequestParams {
        RequestParams {
            body: None,
            path: format!(
                "/accounts/{}/orders/{}",
                params.account_hash, params.order_id
            ),
            method: Method::GET,
            query: None,
        }
    }

    /// Build params for placeOrder operation
    pub fn place_order<'a>(
        params: &PlaceOrderParams<'a>,
    ) -> RequestParams<&'a schwab_api_types::OrderRequest> {
        RequestParams {
            body: Some(params.order),
            path: format!("/accounts/{}/orders", params.account_hash),
            method: Method::POST,
            query: None,
        }
    }

    /// Build params for cancelOrder operation
    pub fn cancel_order(params: &CancelOrderParams) -> RequestParams {
        RequestParams {
            body: None,
            path: format!(
                "/accounts/{}/orders/{}",
                params.account_hash, params.order_id
            ),
            method: Method::DELETE,
            query: None,
        }
    }

    /// Build params for replaceOrder operation
    pub fn replace_order<'a>(
        params: &ReplaceOrderParams<'a>,
    ) -> RequestParams<&'a schwab_api_types::OrderRequest> {
        RequestParams {
            body: Some(params.order),
            path: format!(
                "/accounts/{}/orders/{}",
                params.account_hash, params.order_id
            ),
            method: Method::PUT,
            query: None,
        }
    }

    /// Build params for getOrdersByQueryParam operation
    pub fn get_orders_by_query_param(params: &GetOrdersByQueryParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            body: None,
            path: "/orders".to_string(),
            method: Method::GET,
            query,
        }
    }

    /// Build params for previewOrder operation
    pub fn preview_order<'a>(
        params: &PreviewOrderParams<'a>,
    ) -> RequestParams<&'a schwab_api_types::PreviewOrder> {
        RequestParams {
            body: Some(params.order),
            path: format!("/accounts/{}/previewOrder", params.account_hash),
            method: Method::POST,
            query: None,
        }
    }

    /// Build params for getTransactionsByPathParam operation
    pub fn get_transactions_by_path_param(params: &GetTransactionsByPathParams) -> RequestParams {
        let query = serde_urlencoded::to_string(params).ok();

        RequestParams {
            body: None,
            path: format!("/accounts/{}/transactions", params.account_hash),
            method: Method::GET,
            query,
        }
    }

    /// Build params for getTransactionsById operation
    pub fn get_transactions_by_id(params: &GetTransactionByIdParams) -> RequestParams {
        RequestParams {
            body: None,
            path: format!(
                "/accounts/{}/transactions/{}",
                params.account_hash, params.transaction_id
            ),
            method: Method::GET,
            query: None,
        }
    }

    /// Build params for getUserPreference operation
    pub fn get_user_preference() -> RequestParams {
        RequestParams {
            body: None,
            path: "/userPreference".to_string(),
            method: Method::GET,
            query: None,
        }
    }
}
