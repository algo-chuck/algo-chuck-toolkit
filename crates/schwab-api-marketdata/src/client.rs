use http::{Request, Response};
use schwab_api_core::{HttpClient, HttpError, HttpResponse, RequestParams, SchwabSuccess};
use serde::Serialize;
use serde::de::{DeserializeOwned, Error};

use crate::params::MarketdataParams;

pub struct MarketdataClient<C> {
    pub client: HttpClient<C>,
    base_url: &'static str,
}

impl<C> MarketdataParams for MarketdataClient<C> {}

impl<C> MarketdataClient<C> {
    pub fn new(client: C) -> Self {
        Self {
            client: HttpClient::new(client),
            base_url: "https://api.schwabapi.com/marketdata/v1",
        }
    }

    fn build_url(&self, path: &str, query_string_opt: Option<&str>) -> String {
        let query_prefix = match query_string_opt {
            // Check if the string exists and is not empty.
            Some(value) if !value.is_empty() => format!("?{value}"),
            // If it's None or empty, use an empty string.
            _ => String::new(),
        };

        // Combine base URL, path, and the (optionally prefixed) query string.
        format!("{}{}{}", self.base_url, path, query_prefix)
    }

    pub fn build_request<B: Serialize>(
        &self,
        params: &RequestParams<B>,
    ) -> Result<Request<String>, HttpError> {
        let url = self.build_url(&params.path, params.query.as_deref());
        let bearer_token = format!("Bearer {}", params.access_token);

        // Serialize the body if present
        let final_body = match &params.body {
            Some(body) => serde_json::to_string(body)?,
            None => String::new(),
        };

        Request::builder()
            .uri(url)
            .method(params.method.clone())
            .header("Authorization", bearer_token)
            // .header("Content-Type", "application/json") // Causing 400 error, need to fix for POST
            .body(final_body)
            // The request building error (http::Error) is handled explicitly
            // by mapping it to an appropriate HttpError variant, avoiding the need for
            // a global From<http::Error> implementation.
            .map_err(|e| HttpError::RequestFailed(format!("Request builder failed: {}", e)))
    }

    // This method performs the robust deserialization, logging, and error conversion.
    pub fn parse_ok_response<R: DeserializeOwned>(
        &self,
        response: &Response<String>,
    ) -> Result<R, HttpError> {
        // Perform robust parsing into the GENERIC wrapper type.
        let ok_result = response.json()?;

        // Inspect the result, log the anomaly, and return the final type.
        match ok_result {
            SchwabSuccess::Ok(data) => Ok(data),
            SchwabSuccess::MismatchedResponse(value) => {
                // Log the anomaly: API returned 2xx, but structure was mismatched.
                eprintln!(
                    "WARNING: API returned status {}, but response body was mismatched:\n {:#?}",
                    response.status(),
                    value
                );

                // Treat the unexpected structure as a serialization failure.
                Err(HttpError::SerializationError(
                    // Generate a serde_json error object detailing the issue.
                    serde_json::Error::custom(format!(
                        "Received mismatched {} response structure:\n {:#?}",
                        response.status(),
                        value
                    )),
                ))
            }
        }
    }
}
