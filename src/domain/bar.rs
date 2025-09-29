use serde::{Deserialize, Serialize};

/// Data structure for aggregated financial data, representing a single
/// slice of time with open, high, low, close prices and volume.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bar {
    pub timestamp: String,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32,
}
