use http::{Method, Request};
use std::collections::HashMap;

use schwab_api_core::{AsyncClient, HttpClient, HttpError, HttpResponse};
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

impl<T: AsyncClient> TraderClient<T> {
    pub async fn get_user_preference(
        &self,
        access_token: &str,
    ) -> Result<UserPreference, HttpError> {
        // TODO: Add Trader Manager
        let url = self.build_url("/userPreference", None);
        let bearer_token = format!("Bearer {access_token}");
        let result = Request::builder()
            .uri(url)
            .method(Method::GET)
            .header("Authorization", bearer_token)
            .body(String::new());

        match result {
            Ok(request) => {
                let response = self.client.execute(request).await?;
                let typed = response.json()?;
                Ok(typed)
            }
            Err(_) => Err(HttpError::RequestFailed("Failed to build request".into())),
        }
    }
}
