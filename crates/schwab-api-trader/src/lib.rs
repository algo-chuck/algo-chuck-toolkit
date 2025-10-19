use http::{Method, Request, Response};
use serde::de::{DeserializeOwned, Error};
use std::borrow::Cow;

use schwab_api_core::{
    AsyncClient, HttpClient, HttpError, HttpResponse, RequestParams, SchwabSuccess,
};
use schwab_api_types::UserPreference;

pub struct TraderClient<T> {
    client: HttpClient<T>,
    base_url: String,
}

impl<T> TraderClient<T> {
    pub fn new(client: T) -> Self {
        Self {
            client: HttpClient::new(client),
            base_url: "https://api.schwabapi.com/trader/v1".to_owned(),
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

    fn build_request(&self, params: &RequestParams) -> Result<Request<String>, HttpError> {
        let url = self.build_url(params.path, params.query);
        let bearer_token = format!("Bearer {}", params.access_token);

        // Determine the body content
        let final_body = params
            .body
            .as_ref()
            .unwrap_or(&Cow::from(""))
            .clone()
            .into_owned();

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
}

impl<T> TraderClient<T>
where
    T: AsyncClient,
    HttpError: From<T::Error>,
{
    // This method performs the robust deserialization, logging, and error conversion.
    fn parse_ok_response<R: DeserializeOwned>(response: &Response<String>) -> Result<R, HttpError> {
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

    pub async fn get_user_preference(
        &self,
        access_token: &str,
    ) -> Result<UserPreference, HttpError> {
        let params = RequestParams {
            access_token,
            body: None,
            method: Method::GET,
            path: "/userPreference",
            query: None,
        };

        let request = self.build_request(&params)?;

        // Success path continues immediately.
        let response = self
            .client
            .execute(request)
            .await
            // Use HttpError::from to explicitly tell the compiler the target type.
            .map_err(HttpError::from)?;

        // Use the single helper method to handle deserialization, logging, and error conversion.
        let typed = Self::parse_ok_response(&response)?;
        Ok(typed)
    }
}
