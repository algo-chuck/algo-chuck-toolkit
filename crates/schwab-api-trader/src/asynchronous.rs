use schwab_api_core::{AsyncClient, HttpError, RequestParams};
use schwab_api_types::{Account, AccountNumberHash, UserPreference};
use serde::de::DeserializeOwned;

use crate::client::TraderClient;
use crate::params::TraderParams;

impl<C> TraderClient<C>
where
    C: AsyncClient,
    HttpError: From<C::Error>,
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

    pub async fn get_account_numbers(
        &self,
        access_token: &str,
    ) -> Result<Vec<AccountNumberHash>, HttpError> {
        let params = TraderClient::<C>::get_account_numbers_params(access_token);
        self.fetch(&params).await
    }

    pub async fn get_accounts(
        &self,
        access_token: &str,
        fields: Option<&str>,
    ) -> Result<Vec<Account>, HttpError> {
        let params = TraderClient::<C>::get_accounts_params(access_token, fields);
        self.fetch(&params).await
    }

    pub async fn get_user_preference(
        &self,
        access_token: &str,
    ) -> Result<UserPreference, HttpError> {
        let params = TraderClient::<C>::get_user_preference_params(access_token);
        self.fetch(&params).await
    }
}
