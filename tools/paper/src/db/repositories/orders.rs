// db/repositories/orders.rs
// Implements operations from OpenAPI tag: "Orders"

use schwab_api::types::trader::{
    GetOrdersByPathParams, GetOrdersByQueryParams, Order, OrderRequest,
};
use sqlx::SqlitePool;

use crate::db::{RepositoryError, not_found};

#[derive(Debug, Clone)]
pub struct OrderRepository {
    pool: SqlitePool,
}

impl OrderRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // operationId: placeOrder
    pub async fn place_order(
        &self,
        account_number: &str,
        order_data: &OrderRequest,
    ) -> Result<i64, RepositoryError> {
        let order_data_json = serde_json::to_string(order_data)?;
        let status = "WORKING"; // Initial status

        // Get next order_id (starting from 1001)
        let order_id: i64 =
            sqlx::query_scalar("SELECT COALESCE(MAX(order_id), 1000) + 1 FROM orders")
                .fetch_one(&self.pool)
                .await?;

        sqlx::query(
            "INSERT INTO orders (order_id, account_number, status, order_data, entered_time)
             VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)",
        )
        .bind(order_id)
        .bind(account_number)
        .bind(status)
        .bind(order_data_json)
        .execute(&self.pool)
        .await?;

        Ok(order_id)
    }

    // operationId: getOrder
    pub async fn get_order(&self, order_id: i64) -> Result<Order, RepositoryError> {
        let order_data =
            sqlx::query_scalar::<_, String>("SELECT order_data FROM orders WHERE order_id = ?")
                .bind(order_id)
                .fetch_optional(&self.pool)
                .await?
                .ok_or_else(|| not_found("Order", &order_id.to_string()))?;

        serde_json::from_str(&order_data).map_err(RepositoryError::from)
    }

    // operationId: getOrdersByPathParam
    pub async fn get_orders_by_path_param(
        &self,
        params: &GetOrdersByPathParams<'_>,
    ) -> Result<Vec<Order>, RepositoryError> {
        // Build dynamic query based on provided parameters
        let mut query = String::from("SELECT order_data FROM orders WHERE account_number = ?");
        let mut bind_values: Vec<String> = vec![params.account_hash.to_string()];

        // Add date range filtering (required fields)
        query.push_str(" AND entered_time >= ?");
        bind_values.push(params.from_entered_time.to_string());

        query.push_str(" AND entered_time <= ?");
        bind_values.push(params.to_entered_time.to_string());

        // Add status filtering if provided (optional)
        if let Some(status) = params.status {
            query.push_str(" AND status = ?");
            bind_values.push(status.to_string());
        }

        query.push_str(" ORDER BY entered_time DESC");

        // Add limit if max_results provided (optional)
        if let Some(max_results) = params.max_results {
            query.push_str(&format!(" LIMIT {}", max_results));
        }

        // Execute query with dynamic bindings
        let mut sqlx_query = sqlx::query_scalar::<_, String>(&query);
        for value in bind_values {
            sqlx_query = sqlx_query.bind(value);
        }

        let rows = sqlx_query.fetch_all(&self.pool).await?;

        rows.into_iter()
            .map(|r| serde_json::from_str(&r).map_err(RepositoryError::from))
            .collect()
    }

    // operationId: getOrdersByQueryParam
    pub async fn get_orders_by_query_param(
        &self,
        params: &GetOrdersByQueryParams<'_>,
    ) -> Result<Vec<Order>, RepositoryError> {
        // Build dynamic query (similar to path param but without account filter)
        let mut query = String::from("SELECT order_data FROM orders WHERE 1=1");
        let mut bind_values: Vec<String> = vec![];

        // Add date range filtering (required fields)
        query.push_str(" AND entered_time >= ?");
        bind_values.push(params.from_entered_time.to_string());

        query.push_str(" AND entered_time <= ?");
        bind_values.push(params.to_entered_time.to_string());

        // Add status filtering if provided (optional)
        if let Some(status) = params.status {
            query.push_str(" AND status = ?");
            bind_values.push(status.to_string());
        }

        query.push_str(" ORDER BY entered_time DESC");

        // Add limit if max_results provided (optional)
        if let Some(max_results) = params.max_results {
            query.push_str(&format!(" LIMIT {}", max_results));
        }

        // Execute query with dynamic bindings
        let mut sqlx_query = sqlx::query_scalar::<_, String>(&query);
        for value in bind_values {
            sqlx_query = sqlx_query.bind(value);
        }

        let rows = sqlx_query.fetch_all(&self.pool).await?;

        rows.into_iter()
            .map(|r| serde_json::from_str(&r).map_err(RepositoryError::from))
            .collect()
    }

    // operationId: cancelOrder
    pub async fn cancel_order(&self, order_id: i64) -> Result<(), RepositoryError> {
        let status = "CANCELED";

        sqlx::query(
            "UPDATE orders 
             SET status = ?, close_time = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
             WHERE order_id = ?",
        )
        .bind(status)
        .bind(order_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // operationId: replaceOrder
    pub async fn replace_order(
        &self,
        order_id: i64,
        new_order_data: &OrderRequest,
    ) -> Result<i64, RepositoryError> {
        // Cancel old order
        self.cancel_order(order_id).await?;

        // Get account_number from old order
        let old_order = self.get_order(order_id).await?;

        let account_number = old_order
            .account_number
            .ok_or_else(|| not_found("Order", &order_id.to_string()))?
            .to_string();

        // Place new order
        self.place_order(&account_number, new_order_data).await
    }

    // operationId: previewOrder
    // Note: Preview doesn't persist to database, handled in service layer

    // Additional helper methods

    pub async fn update_status(&self, order_id: i64, status: &str) -> Result<(), RepositoryError> {
        sqlx::query(
            "UPDATE orders 
             SET status = ?, updated_at = CURRENT_TIMESTAMP
             WHERE order_id = ?",
        )
        .bind(status)
        .bind(order_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update(&self, order_id: i64, order_data: &Order) -> Result<(), RepositoryError> {
        let order_data_json = serde_json::to_string(order_data)?;
        let status = &order_data
            .status
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "UNKNOWN".to_string());

        sqlx::query(
            "UPDATE orders 
             SET order_data = ?, status = ?, updated_at = CURRENT_TIMESTAMP
             WHERE order_id = ?",
        )
        .bind(order_data_json)
        .bind(status)
        .bind(order_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!("./src/db/migrations")
            .run(&pool)
            .await
            .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_order_id_starts_at_1001() {
        let pool = setup_test_db().await;
        let repo = OrderRepository::new(pool);

        // First order should have ID 1001
        let order_req = OrderRequest::default();
        let order_id = repo.place_order("12345", &order_req).await.unwrap();
        assert_eq!(order_id, 1001);
    }
}
