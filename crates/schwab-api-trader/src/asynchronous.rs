use schwab_api_core::{AsyncClient, HttpError, RequestParams};
use schwab_api_types::{Account, AccountNumberHash, Order, Transaction, UserPreference};
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

    // Accounts

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

    pub async fn get_account(
        &self,
        access_token: &str,
        account_number: &str,
        fields: Option<&str>,
    ) -> Result<Account, HttpError> {
        let params = TraderClient::<C>::get_account_params(access_token, account_number, fields);
        self.fetch(&params).await
    }

    // Orders

    pub async fn get_orders_by_path_param(
        &self,
        access_token: &str,
        account_number: &str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i64>,
        status: Option<&str>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderClient::<C>::get_orders_by_path_param_params(
            access_token,
            account_number,
            from_entered_time,
            to_entered_time,
            max_results,
            status,
        );
        self.fetch(&params).await
    }

    pub async fn get_order(
        &self,
        access_token: &str,
        account_number: &str,
        order_id: i64,
    ) -> Result<Order, HttpError> {
        let params = TraderClient::<C>::get_order_params(access_token, account_number, order_id);
        self.fetch(&params).await
    }

    pub async fn get_orders_by_query_param(
        &self,
        access_token: &str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i64>,
        status: Option<&str>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderClient::<C>::get_orders_by_query_param_params(
            access_token,
            from_entered_time,
            to_entered_time,
            max_results,
            status,
        );
        self.fetch(&params).await
    }

    // Transactions

    pub async fn get_transactions_by_path_param(
        &self,
        access_token: &str,
        account_number: &str,
        start_date: &str,
        end_date: &str,
        types: &str,
        symbol: Option<&str>,
    ) -> Result<Vec<Transaction>, HttpError> {
        let params = TraderClient::<C>::get_transactions_by_path_param_params(
            access_token,
            account_number,
            start_date,
            end_date,
            types,
            symbol,
        );
        self.fetch(&params).await
    }

    pub async fn get_transactions_by_id(
        &self,
        access_token: &str,
        account_number: &str,
        transaction_id: i64,
    ) -> Result<Vec<Transaction>, HttpError> {
        let params = TraderClient::<C>::get_transactions_by_id_params(
            access_token,
            account_number,
            transaction_id,
        );
        self.fetch(&params).await
    }

    // User Preference

    pub async fn get_user_preference(
        &self,
        access_token: &str,
    ) -> Result<UserPreference, HttpError> {
        let params = TraderClient::<C>::get_user_preference_params(access_token);
        self.fetch(&params).await
    }
}
