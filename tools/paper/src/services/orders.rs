//! Order service - business logic for order operations
//!
//! Thin CRUD wrapper around OrderRepository with input validation.

use crate::db::repositories::{OrderError, OrderRepository};
use schwab_api::types::trader::{
    CancelOrderParams, GetOrderParams, GetOrdersByPathParams, GetOrdersByQueryParams, Order,
    OrderRequest, PlaceOrderParams, PreviewOrder, PreviewOrderParams, ReplaceOrderParams,
};

/// Errors that can occur in order service operations
#[derive(Debug, thiserror::Error)]
pub enum OrderServiceError {
    #[error("Order not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Repository error: {0}")]
    Repository(#[from] OrderError),
}

/// Service for order operations
pub struct OrderService {
    repository: OrderRepository,
}

impl OrderService {
    /// Create a new order service
    pub fn new(repository: OrderRepository) -> Self {
        Self { repository }
    }

    /// Place a new order
    ///
    /// Maps to: POST /trader/v1/accounts/{accountNumber}/orders
    pub async fn place_order(
        &self,
        params: PlaceOrderParams<'_>,
        order_request: OrderRequest,
    ) -> Result<i64, OrderServiceError> {
        // Validate account hash is not empty
        if params.account_hash.trim().is_empty() {
            return Err(OrderServiceError::InvalidInput(
                "account_hash cannot be empty".to_string(),
            ));
        }

        // Validate order request has required fields
        self.validate_order_request(&order_request)?;

        // Use account_hash for now (account_number in OrderRequest is i64, not string)
        let account_number = params.account_hash;

        self.repository
            .place_order(account_number, &order_request)
            .await
            .map_err(OrderServiceError::from)
    }

    /// Get a specific order by ID
    ///
    /// Maps to: GET /trader/v1/accounts/{accountNumber}/orders/{orderId}
    pub async fn get_order(&self, params: GetOrderParams<'_>) -> Result<Order, OrderServiceError> {
        // Validate account hash is not empty
        if params.account_hash.trim().is_empty() {
            return Err(OrderServiceError::InvalidInput(
                "account_hash cannot be empty".to_string(),
            ));
        }

        self.repository
            .get_order(params.order_id)
            .await
            .map_err(OrderServiceError::from)
    }

    /// Get orders for an account by path parameters
    ///
    /// Maps to: GET /trader/v1/accounts/{accountNumber}/orders
    pub async fn get_orders_by_path(
        &self,
        params: GetOrdersByPathParams<'_>,
    ) -> Result<Vec<Order>, OrderServiceError> {
        // Validate account hash is not empty
        if params.account_hash.trim().is_empty() {
            return Err(OrderServiceError::InvalidInput(
                "account_hash cannot be empty".to_string(),
            ));
        }

        self.repository
            .get_orders_by_path_param(&params)
            .await
            .map_err(OrderServiceError::from)
    }

    /// Get orders across all accounts by query parameters
    ///
    /// Maps to: GET /trader/v1/orders
    pub async fn get_orders_by_query(
        &self,
        params: GetOrdersByQueryParams<'_>,
    ) -> Result<Vec<Order>, OrderServiceError> {
        self.repository
            .get_orders_by_query_param(&params)
            .await
            .map_err(OrderServiceError::from)
    }

    /// Cancel an order
    ///
    /// Maps to: DELETE /trader/v1/accounts/{accountNumber}/orders/{orderId}
    pub async fn cancel_order(
        &self,
        params: CancelOrderParams<'_>,
    ) -> Result<(), OrderServiceError> {
        // Validate account hash is not empty
        if params.account_hash.trim().is_empty() {
            return Err(OrderServiceError::InvalidInput(
                "account_hash cannot be empty".to_string(),
            ));
        }

        self.repository
            .cancel_order(params.order_id)
            .await
            .map_err(OrderServiceError::from)
    }

    /// Replace an existing order
    ///
    /// Maps to: PUT /trader/v1/accounts/{accountNumber}/orders/{orderId}
    pub async fn replace_order(
        &self,
        params: ReplaceOrderParams<'_>,
        order_request: OrderRequest,
    ) -> Result<i64, OrderServiceError> {
        // Validate account hash is not empty
        if params.account_hash.trim().is_empty() {
            return Err(OrderServiceError::InvalidInput(
                "account_hash cannot be empty".to_string(),
            ));
        }

        // Validate order request has required fields
        self.validate_order_request(&order_request)?;

        self.repository
            .replace_order(params.order_id, &order_request)
            .await
            .map_err(OrderServiceError::from)
    }

    /// Preview an order without placing it
    ///
    /// Maps to: POST /trader/v1/accounts/{accountNumber}/previewOrder
    ///
    /// Note: This method does not persist anything to the database.
    /// It validates the order and calculates estimated costs/commissions.
    pub async fn preview_order(
        &self,
        params: PreviewOrderParams<'_>,
        order_request: OrderRequest,
    ) -> Result<PreviewOrder, OrderServiceError> {
        // Validate account hash is not empty
        if params.account_hash.trim().is_empty() {
            return Err(OrderServiceError::InvalidInput(
                "account_hash cannot be empty".to_string(),
            ));
        }

        // Validate order request has required fields
        self.validate_order_request(&order_request)?;

        // TODO: Calculate estimated costs, commissions, and order validation
        // For now, return a basic preview structure
        // This will be expanded in Phase 4 with proper commission calculation

        Ok(PreviewOrder {
            order_id: None,                // Preview doesn't have an order ID yet
            order_strategy: None,          // TODO: Convert OrderRequest to OrderStrategy
            order_validation_result: None, // TODO: Add validation result
            commission_and_fee: None,      // TODO: Calculate commissions and fees
        })
    }

    // Validation helpers

    fn validate_order_request(&self, order: &OrderRequest) -> Result<(), OrderServiceError> {
        // Basic validation - ensure session, duration, and order_type are present
        if order.session.is_none() {
            return Err(OrderServiceError::InvalidInput(
                "session is required".to_string(),
            ));
        }

        if order.duration.is_none() {
            return Err(OrderServiceError::InvalidInput(
                "duration is required".to_string(),
            ));
        }

        if order.order_type.is_none() {
            return Err(OrderServiceError::InvalidInput(
                "order_type is required".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests will be added with in-memory database setup
    #[tokio::test]
    async fn test_placeholder() {
        // TODO: Implement tests with :memory: database
    }
}
