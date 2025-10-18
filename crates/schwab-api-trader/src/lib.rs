use std::collections::HashMap;

use schwab_api_core::{AsyncClient, HttpClient, HttpError, HttpMethod, HttpRequest};
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
    http_client: HttpClient<T>,
    base_url: String,
}

impl<T> TraderClient<T> {
    pub fn new(http_client: T) -> Self {
        Self {
            http_client: HttpClient::new(http_client),
            base_url: "https://api.schwabapi.com/trader/v1".to_owned(),
        }
    }

    pub fn with_base_url(http_client: T, base_url: impl Into<String>) -> Self {
        Self {
            http_client: HttpClient::new(http_client),
            base_url: base_url.into(),
        }
    }

    fn build_url(&self, path: &str, query: Option<&QueryParams>) -> String {
        let query_string = query.map(|q| q.to_query_string()).unwrap_or_default();
        format!("{}{}{}", self.base_url, path, query_string)
    }
}

impl<T: AsyncClient + Send + Sync> TraderClient<T> {
    pub async fn get_user_preference(
        &self,
        access_token: &str,
    ) -> Result<UserPreference, HttpError> {
        let url = self.build_url("/userPreference", None);
        let token = format!("Bearer {}", access_token);
        let request = HttpRequest::new(HttpMethod::Get, url).header("Authorization", token);
        let response = self.http_client.execute_async(request).await?;
        // Deserialize the response body directly into the typed model
        let typed = response.json::<UserPreference>()?;
        Ok(typed)
    }
}
