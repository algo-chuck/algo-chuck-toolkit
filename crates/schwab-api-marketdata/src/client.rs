use schwab_api_core::{ApiClient, ApiConfig};

use crate::params::MarketdataParams;

/// Configuration for the Marketdata API
pub struct MarketdataConfig;

impl ApiConfig for MarketdataConfig {
    fn base_url() -> &'static str {
        "https://api.schwabapi.com/marketdata/v1"
    }
}

/// MarketdataClient wraps ApiClient configured with MarketdataConfig
pub struct MarketdataClient<C> {
    inner: ApiClient<C, MarketdataConfig>,
}

impl<C> MarketdataClient<C> {
    pub fn new(client: C) -> Self {
        Self {
            inner: ApiClient::new(client),
        }
    }

    /// Access the inner ApiClient for direct operations
    pub fn inner(&self) -> &ApiClient<C, MarketdataConfig> {
        &self.inner
    }

    /// Access the inner ApiClient mutably
    pub fn inner_mut(&mut self) -> &mut ApiClient<C, MarketdataConfig> {
        &mut self.inner
    }
}

impl<C> MarketdataParams for MarketdataClient<C> {}

// Implement Deref to allow calling ApiClient methods directly
impl<C> std::ops::Deref for MarketdataClient<C> {
    type Target = ApiClient<C, MarketdataConfig>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut to allow mutable access to ApiClient methods
impl<C> std::ops::DerefMut for MarketdataClient<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
