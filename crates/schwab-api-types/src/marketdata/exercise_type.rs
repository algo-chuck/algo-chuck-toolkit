use serde::{Deserialize, Serialize};

/// ExerciseType : option contract exercise type America or European
///
/// **Variants:**
/// - `A`
/// - `E`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExerciseType {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "E")]
    E,
}

impl std::fmt::Display for ExerciseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::E => write!(f, "E"),
        }
    }
}

impl Default for ExerciseType {
    fn default() -> ExerciseType {
        Self::A
    }
}
