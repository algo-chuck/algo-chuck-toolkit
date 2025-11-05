use schwab_api_core::{ApiClient, ApiConfig};

use crate::params::TraderParams;

/// Configuration for the Trader API
pub struct TraderConfig;

impl ApiConfig for TraderConfig {
    fn base_url() -> &'static str {
        "https://api.schwabapi.com/trader/v1"
    }
}

/// TraderClient wraps ApiClient configured with TraderConfig
pub struct TraderClient<C> {
    inner: ApiClient<C, TraderConfig>,
}

impl<C> TraderClient<C> {
    pub fn new(client: C) -> Self {
        Self {
            inner: ApiClient::new(client),
        }
    }

    /// Access the inner ApiClient for direct operations
    pub fn inner(&self) -> &ApiClient<C, TraderConfig> {
        &self.inner
    }

    /// Access the inner ApiClient mutably
    pub fn inner_mut(&mut self) -> &mut ApiClient<C, TraderConfig> {
        &mut self.inner
    }
}

impl<C> TraderParams for TraderClient<C> {}

// Implement Deref to allow calling ApiClient methods directly
impl<C> std::ops::Deref for TraderClient<C> {
    type Target = ApiClient<C, TraderConfig>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut to allow mutable access to ApiClient methods
impl<C> std::ops::DerefMut for TraderClient<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
