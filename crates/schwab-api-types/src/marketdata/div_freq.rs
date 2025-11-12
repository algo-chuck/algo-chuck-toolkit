use serde_repr::{Deserialize_repr, Serialize_repr};

/// Dividend frequency 1 – once a year or annually 2 – 2x a year or semi-annually 3 - 3x a year (ex. ARCO, EBRPF) 4 – 4x a year or quarterly 6 - 6x per yr or every other month 11 – 11x a year (ex. FBND, FCOR) 12 – 12x a year or monthly
#[repr(i64)]
/// Dividend frequency 1 – once a year or annually 2 – 2x a year or semi-annually 3 - 3x a year (ex. ARCO, EBRPF) 4 – 4x a year or quarterly 6 - 6x per yr or every other month 11 – 11x a year (ex. FBND, FCOR) 12 – 12x a year or monthly
///
/// **Variants:**
/// - `1`
/// - `2`
/// - `3`
/// - `4`
/// - `6`
/// - `11`
/// - `12`
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum DivFreq {
    Variant1 = 1,
    Variant2 = 2,
    Variant3 = 3,
    Variant4 = 4,
    Variant6 = 6,
    Variant11 = 11,
    Variant12 = 12,
}

impl std::fmt::Display for DivFreq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Variant1 => "1",
                Self::Variant2 => "2",
                Self::Variant3 => "3",
                Self::Variant4 => "4",
                Self::Variant6 => "6",
                Self::Variant11 => "11",
                Self::Variant12 => "12",
            }
        )
    }
}
impl Default for DivFreq {
    fn default() -> DivFreq {
        Self::Variant1
    }
}
