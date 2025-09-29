use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tick {
    pub timestamp: String,
    pub symbol: String,
    pub price: f32,
    pub volume: f32,
}
