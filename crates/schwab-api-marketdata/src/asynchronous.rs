use schwab_api_core::{AsyncClient, HttpError, RequestParams};
use schwab_api_types::QuoteResponseObject;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::client::MarketdataClient;
use crate::params::MarketdataParams;

impl<C> MarketdataClient<C>
where
    C: AsyncClient,
    HttpError: From<C::Error>,
{
    async fn fetch<'a, R, B>(&self, params: &'a RequestParams<'a, B>) -> Result<R, HttpError>
    where
        R: DeserializeOwned,
        B: Serialize,
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

    // Quotes

    /// Get quotes for multiple symbols
    pub async fn get_quotes(
        &self,
        access_token: &str,
        symbols: &str,
        fields: Option<&str>,
        indicative: Option<bool>,
    ) -> Result<HashMap<String, QuoteResponseObject>, HttpError> {
        let params =
            MarketdataClient::<C>::get_quotes_params(access_token, symbols, fields, indicative);
        self.fetch(&params).await
    }

    /// Get quote for a single symbol
    pub async fn get_quote(
        &self,
        access_token: &str,
        symbol: &str,
        fields: Option<&str>,
    ) -> Result<HashMap<String, QuoteResponseObject>, HttpError> {
        let params = MarketdataClient::<C>::get_quote_params(access_token, symbol, fields);
        self.fetch(&params).await
    }

    // Option Chains

    // Options Expiration Chain

    // Price History

    // Movers

    // Market Hours

    // Instruments
}
