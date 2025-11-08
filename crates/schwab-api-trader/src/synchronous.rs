use schwab_api_core::{HttpError, SyncHttpClient};
use schwab_api_types::{
    Account, AccountNumberHash, Order, OrderRequest, PreviewOrder, Transaction, UserPreference,
};

use crate::client::TraderClient;
use crate::params::TraderParams;

impl<C> TraderClient<C>
where
    C: SyncHttpClient,
    HttpError: From<C::Error>,
{
    // Accounts

    pub fn get_account_numbers_sync(
        &self,
        access_token: &str,
    ) -> Result<Vec<AccountNumberHash>, HttpError> {
        let params = TraderClient::<C>::get_account_numbers_params(access_token);
        self.fetch_sync(&params)
    }

    pub fn get_accounts_sync(
        &self,
        access_token: &str,
        fields: Option<&str>,
    ) -> Result<Vec<Account>, HttpError> {
        let params = TraderClient::<C>::get_accounts_params(access_token, fields);
        self.fetch_sync(&params)
    }

    pub fn get_account_sync(
        &self,
        access_token: &str,
        account_number: &str,
        fields: Option<&str>,
    ) -> Result<Account, HttpError> {
        let params = TraderClient::<C>::get_account_params(access_token, account_number, fields);
        self.fetch_sync(&params)
    }

    // Orders

    pub fn get_orders_by_path_sync(
        &self,
        access_token: &str,
        account_number: &str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i64>,
        status: Option<&str>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderClient::<C>::get_orders_by_path_params(
            access_token,
            account_number,
            from_entered_time,
            to_entered_time,
            max_results,
            status,
        );
        self.fetch_sync(&params)
    }

    pub fn get_order_sync(
        &self,
        access_token: &str,
        account_number: &str,
        order_id: i64,
    ) -> Result<Order, HttpError> {
        let params = TraderClient::<C>::get_order_params(access_token, account_number, order_id);
        self.fetch_sync(&params)
    }

    pub fn get_orders_by_query_sync(
        &self,
        access_token: &str,
        from_entered_time: &str,
        to_entered_time: &str,
        max_results: Option<i64>,
        status: Option<&str>,
    ) -> Result<Vec<Order>, HttpError> {
        let params = TraderClient::<C>::get_orders_by_query_params(
            access_token,
            from_entered_time,
            to_entered_time,
            max_results,
            status,
        );
        self.fetch_sync(&params)
    }

    pub fn place_order_sync(
        &self,
        access_token: &str,
        account_number: &str,
        order: &OrderRequest,
    ) -> Result<(), HttpError> {
        let params = TraderClient::<C>::place_order_params(access_token, account_number, order);
        self.execute_sync(&params)
    }

    pub fn cancel_order_sync(
        &self,
        access_token: &str,
        account_number: &str,
        order_id: i64,
    ) -> Result<(), HttpError> {
        let params = TraderClient::<C>::cancel_order_params(access_token, account_number, order_id);
        self.execute_sync(&params)
    }

    pub fn replace_order_sync(
        &self,
        access_token: &str,
        account_number: &str,
        order_id: i64,
        order: &OrderRequest,
    ) -> Result<(), HttpError> {
        let params =
            TraderClient::<C>::replace_order_params(access_token, account_number, order_id, order);
        self.execute_sync(&params)
    }

    pub fn preview_order_sync(
        &self,
        access_token: &str,
        account_number: &str,
        preview: &PreviewOrder,
    ) -> Result<PreviewOrder, HttpError> {
        let params = TraderClient::<C>::preview_order_params(access_token, account_number, preview);
        self.fetch_sync(&params)
    }

    // Transactions

    pub fn get_transactions_by_path_param_sync(
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
        self.fetch_sync(&params)
    }

    pub fn get_transactions_by_id_sync(
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
        self.fetch_sync(&params)
    }

    // User Preferences

    pub fn get_user_preference_sync(
        &self,
        access_token: &str,
    ) -> Result<UserPreference, HttpError> {
        let params = TraderClient::<C>::get_user_preference_params(access_token);
        self.fetch_sync(&params)
    }
}
