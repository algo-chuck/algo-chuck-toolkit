// Repository module
// Re-exports all repository implementations

pub mod accounts;
pub mod orders;
pub mod transactions;
pub mod user_preference;

pub use accounts::AccountRepository;
pub use orders::OrderRepository;
pub use transactions::TransactionRepository;
pub use user_preference::UserPreferenceRepository;

// Re-export the unified RepositoryError
pub use crate::db::RepositoryError;
