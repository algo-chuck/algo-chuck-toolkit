use http::{Method, Request};
use serde::de::{DeserializeOwned, Error};
use std::collections::HashMap;

use schwab_api_core::{AsyncClient, HttpClient, HttpError, HttpResponse, Response, SchwabSuccess};
use schwab_api_types::UserPreference;

#[derive(Debug, Default)]
pub struct QueryParams {
    params: HashMap<String, String>,
}

impl QueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    pub fn to_query_string(&self) -> String {
        if self.params.is_empty() {
            String::new()
        } else {
            let params: Vec<String> = self
                .params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            format!("?{}", params.join("&"))
        }
    }
}

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

    fn build_url(&self, path: &str, query: Option<&QueryParams>) -> String {
        let query_string = query.map(|q| q.to_query_string()).unwrap_or_default();
        format!("{}{}{}", self.base_url, path, query_string)
    }
}

impl<T> TraderClient<T>
where
    T: AsyncClient,
    HttpError: From<T::Error>,
{
    // This method performs the robust deserialization, logging, and error conversion.
    fn parse_ok_response<R: DeserializeOwned>(response: &Response) -> Result<R, HttpError> {
        // Perform robust parsing into the GENERIC wrapper type.
        let ok_result = response.json()?;

        // Inspect the result, log the anomaly, and return the final type.
        match ok_result {
            SchwabSuccess::Ok(data) => Ok(data),
            SchwabSuccess::MismatchedResponse(value) => {
                // Log the anomaly: API returned 2xx, but structure was mismatched
                eprintln!(
                    "WARNING: API returned status {}, but response body was mismatched:\n {:#?}",
                    response.status(),
                    value
                );

                // Treat the unexpected structure as a serialization failure
                Err(HttpError::SerializationError(
                    // Generate a serde_json error object detailing the issue
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
    ) -> Result<Vec<UserPreference>, HttpError> {
        // TODO: Add Trader Manager
        let url = self.build_url("/userPreference", None);
        let bearer_token = format!("Bearer {access_token}");

        // The request building error (http::Error) is now handled explicitly
        // by mapping it to an appropriate HttpError variant, avoiding the need for
        // a global From<http::Error> implementation.
        let request = Request::builder()
            .uri(url)
            .method(Method::GET)
            .header("Authorization", bearer_token)
            .body(String::new())
            .map_err(|e| HttpError::RequestFailed(format!("Request builder failed: {}", e)))?;

        // Success path continues immediately:
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
