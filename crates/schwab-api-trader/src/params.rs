use http::Method;

use schwab_api_core::RequestParams;
use schwab_api_types::{OrderRequest, PreviewOrder};

pub trait TraderParams {
    // Accounts
    fn get_account_numbers_params<'a>(access_token: &'a str) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: "/accounts/accountNumbers",
            method: Method::GET,
            query: None,
        }
    }

    fn get_accounts_params<'a>(access_token: &'a str, fields: Option<&str>) -> RequestParams<'a> {
        let query = fields.map(|f| Box::leak(format!("fields={}", f).into_boxed_str()) as &str);
        RequestParams {
            access_token,
            body: None,
            path: "/accounts",
            method: Method::GET,
            query,
        }
    }

    fn get_account_params<'a>(
        access_token: &'a str,
        account_number: &str,
        fields: Option<&str>,
    ) -> RequestParams<'a> {
        let path = Box::leak(format!("/accounts/{}", account_number).into_boxed_str()) as &str;
        let query = fields.map(|f| Box::leak(format!("fields={}", f).into_boxed_str()) as &str);
        RequestParams {
            access_token,
            body: None,
            path,
            method: Method::GET,
            query,
        }
    }

    // Orders
    fn get_orders_by_path_param_params<'a>(
        access_token: &'a str,
        account_number: &str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i64>,
        status: Option<&str>,
    ) -> RequestParams<'a> {
        let path =
            Box::leak(format!("/accounts/{}/orders", account_number).into_boxed_str()) as &str;
        let mut query_parts = vec![
            format!("fromEnteredTime={}", from_entered_time),
            format!("toEnteredTime={}", to_entered_time),
        ];
        if let Some(max) = max_results {
            query_parts.push(format!("maxResults={}", max));
        }
        if let Some(s) = status {
            query_parts.push(format!("status={}", s));
        }
        let query = Box::leak(query_parts.join("&").into_boxed_str()) as &str;
        RequestParams {
            access_token,
            body: None,
            path,
            method: Method::GET,
            query: Some(query),
        }
    }

    fn place_order_params<'a>(
        access_token: &'a str,
        account_number: &str,
        order: &'a OrderRequest,
    ) -> RequestParams<'a, &'a OrderRequest> {
        let path =
            Box::leak(format!("/accounts/{}/orders", account_number).into_boxed_str()) as &str;
        RequestParams {
            access_token,
            body: Some(order),
            path,
            method: Method::POST,
            query: None,
        }
    }

    fn get_order_params<'a>(
        access_token: &'a str,
        account_number: &str,
        order_id: i64,
    ) -> RequestParams<'a> {
        let path =
            Box::leak(format!("/accounts/{}/orders/{}", account_number, order_id).into_boxed_str())
                as &str;
        RequestParams {
            access_token,
            body: None,
            path,
            method: Method::GET,
            query: None,
        }
    }

    fn cancel_order_params<'a>(
        access_token: &'a str,
        account_number: &str,
        order_id: i64,
    ) -> RequestParams<'a> {
        let path =
            Box::leak(format!("/accounts/{}/orders/{}", account_number, order_id).into_boxed_str())
                as &str;
        RequestParams {
            access_token,
            body: None,
            path,
            method: Method::DELETE,
            query: None,
        }
    }

    fn replace_order_params<'a>(
        access_token: &'a str,
        account_number: &str,
        order_id: i64,
        order: &'a OrderRequest,
    ) -> RequestParams<'a, &'a OrderRequest> {
        let path =
            Box::leak(format!("/accounts/{}/orders/{}", account_number, order_id).into_boxed_str())
                as &str;
        RequestParams {
            access_token,
            body: Some(order),
            path,
            method: Method::PUT,
            query: None,
        }
    }

    fn get_orders_by_query_param_params<'a>(
        access_token: &'a str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i64>,
        status: Option<&str>,
    ) -> RequestParams<'a> {
        let mut query_parts = vec![
            format!("fromEnteredTime={}", from_entered_time),
            format!("toEnteredTime={}", to_entered_time),
        ];
        if let Some(max) = max_results {
            query_parts.push(format!("maxResults={}", max));
        }
        if let Some(s) = status {
            query_parts.push(format!("status={}", s));
        }
        let query = Box::leak(query_parts.join("&").into_boxed_str()) as &str;
        RequestParams {
            access_token,
            body: None,
            path: "/orders",
            method: Method::GET,
            query: Some(query),
        }
    }

    fn preview_order_params<'a>(
        access_token: &'a str,
        account_number: &str,
        preview: &'a PreviewOrder,
    ) -> RequestParams<'a, &'a PreviewOrder> {
        let path = Box::leak(format!("/accounts/{}/previewOrder", account_number).into_boxed_str())
            as &str;
        RequestParams {
            access_token,
            body: Some(preview),
            path,
            method: Method::POST,
            query: None,
        }
    }

    // Transactions
    fn get_transactions_by_path_param_params<'a>(
        access_token: &'a str,
        account_number: &str,
        start_date: &str,
        end_date: &str,
        types: &str,
        symbol: Option<&str>,
    ) -> RequestParams<'a> {
        let path = Box::leak(format!("/accounts/{}/transactions", account_number).into_boxed_str())
            as &str;
        let mut query_parts = vec![
            format!("startDate={}", start_date),
            format!("endDate={}", end_date),
            format!("types={}", types),
        ];
        if let Some(sym) = symbol {
            query_parts.push(format!("symbol={}", sym));
        }
        let query = Box::leak(query_parts.join("&").into_boxed_str()) as &str;
        RequestParams {
            access_token,
            body: None,
            path,
            method: Method::GET,
            query: Some(query),
        }
    }

    fn get_transactions_by_id_params<'a>(
        access_token: &'a str,
        account_number: &str,
        transaction_id: i64,
    ) -> RequestParams<'a> {
        let path = Box::leak(
            format!(
                "/accounts/{}/transactions/{}",
                account_number, transaction_id
            )
            .into_boxed_str(),
        ) as &str;
        RequestParams {
            access_token,
            body: None,
            path,
            method: Method::GET,
            query: None,
        }
    }

    // User Preference
    fn get_user_preference_params<'a>(access_token: &'a str) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: "/userPreference",
            method: Method::GET,
            query: None,
        }
    }
}
