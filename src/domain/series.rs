use serde::{Deserialize, Serialize};

use crate::domain::{Bar, Indicator, Indicators, Interval, Resolution, calculate};

#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub symbol: String,
    pub resolution: Resolution,
    pub interval: Interval,
    pub data: Vec<Bar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedSeries {
    pub symbol: String,
    pub resolution: Resolution,
    pub interval: Interval,
    pub data: Vec<Bar>,
    pub indicators: Option<Vec<Indicator>>,
}

impl Series {
    pub fn new(symbol: String, resolution: Resolution, interval: Interval, data: Vec<Bar>) -> Self {
        Self {
            symbol,
            resolution,
            interval,
            data,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn enhance_with(&self, indicators: &Vec<Indicators>) -> EnhancedSeries {
        EnhancedSeries {
            symbol: self.symbol.clone(),
            resolution: self.resolution.clone(),
            interval: self.interval,
            data: self.data.clone(),
            indicators: Some(
                indicators
                    .iter()
                    .map(|ind| Indicator {
                        name: ind.name().to_string(),
                        params: ind.params().clone(),
                        data: calculate(ind.name(), &self.data, &ind.params()),
                    })
                    .collect(),
            ),
        }
    }
}
