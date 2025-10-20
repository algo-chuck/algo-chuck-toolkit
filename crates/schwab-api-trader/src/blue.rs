use schwab_api_core::{AsyncClient, HttpError, RequestParams};
use schwab_api_types::UserPreference;
use serde::de::DeserializeOwned;

use crate::client::TraderClient;
use crate::params::TraderParams;

impl<T> TraderClient<T>
where
    T: AsyncClient,
    HttpError: From<T::Error>,
{
    async fn fetch<'a, R>(&self, params: &'a RequestParams<'a>) -> Result<R, HttpError>
    where
        R: DeserializeOwned,
    {
        let request = self.build_request(params)?;

        // Success path continues immediately.
        let response = self
            .client
            .execute(request)
            .await
            // Use HttpError::from to explicitly tell the compiler the target type.
            .map_err(HttpError::from)?;

        // Use the single helper method to handle deserialization, logging, and error conversion.
        let typed = self.parse_ok_response(&response)?;
        Ok(typed)
    }

    pub async fn get_user_preference(
        &self,
        access_token: &str,
    ) -> Result<UserPreference, HttpError> {
        let params = TraderClient::<T>::user_preference_params(access_token);
        self.fetch(&params).await
    }
}
