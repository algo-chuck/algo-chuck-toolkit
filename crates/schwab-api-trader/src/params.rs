use http::Method;

use schwab_api_core::RequestParams;

pub trait TraderParams {
    fn user_preference_params<'a>(access_token: &'a str) -> RequestParams<'a> {
        RequestParams {
            access_token,
            body: None,
            path: "/userPreference",
            method: Method::GET,
            query: None,
        }
    }
}
