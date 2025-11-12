use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "activityType")]
pub enum OrderActivity {
    #[serde(rename = "EXECUTION")]
    Execution,
    #[serde(rename = "ORDER_ACTION")]
    OrderAction,
}

impl Default for OrderActivity {
    fn default() -> OrderActivity {
        Self::Execution
    }
}

/// Represents account and trading information.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExecutionType {
    #[serde(rename = "FILL")]
    Fill,
}

impl Default for ExecutionType {
    fn default() -> ExecutionType {
        Self::Fill
    }
}
