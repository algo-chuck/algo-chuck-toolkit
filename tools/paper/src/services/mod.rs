//! Service layer for paper trader
//!
//! This layer provides business logic and validation on top of the repository layer.
//! Services are thin CRUD wrappers that validate inputs and delegate to repositories.
//!
//! Error handling: Services use internal error types that will be converted to
//! OpenAPI ServiceError types in Phase 5 (handler layer).

pub mod accounts;
pub use accounts::{AccountService, AccountServiceError};

pub mod orders;
pub use orders::{OrderService, OrderServiceError};

pub mod transactions;
pub use transactions::{TransactionService, TransactionServiceError};

pub mod user_preference;
pub use user_preference::{UserPreferenceService, UserPreferenceServiceError};
