use std::collections::VecDeque;

use crate::domain::{Bar, Calculation};

/// XAverage
///
/// XAverage uses a smoothing factor to calculate the average based on the closing prices.
///
/// XAverage = (Current Price - Previous XAverage) * Factor + Previous XAverage
///
/// where:
/// - Factor = 2 / (Period + 1)
/// - `Period` is the number of periods over which the average is calculated.
/// - `Current Price` is the current closing price.
/// - `Previous XAverage` is the last calculated XAverage value.
/// - Initializes the first value of XAverage to the first closing price.
pub struct XAverage;

impl XAverage {
    pub fn calculate_xaverage(&self, data: &Vec<Bar>, period: usize) -> VecDeque<f32> {
        let mut xavg: VecDeque<f32> = VecDeque::new();
        let factor = 2.0 / (period as f32 + 1.0);

        for (i, bar) in data.iter().enumerate() {
            if i == 0 {
                xavg.push_back(bar.close); // First value is just the close price
            } else {
                let prev_xaverage = *xavg.back().unwrap();
                let new_xaverage = (bar.close - prev_xaverage) * factor + prev_xaverage;
                xavg.push_back(new_xaverage);
            }
        }

        xavg
    }
}

impl Calculation for XAverage {
    fn calculate(&self, data: &Vec<Bar>, params: &Vec<f32>) -> VecDeque<f32> {
        let period = params.get(0).cloned().unwrap_or(14.0) as usize;
        self.calculate_xaverage(data, period)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Bar;
    use crate::indicators::xaverage::XAverage;

    #[test]
    fn test_xaverage() {
        let xavg = XAverage;
        let data = vec![
            Bar {
                timestamp: "2023-01-01T00:00:00Z".to_string(),
                open: 100.0,
                high: 105.0,
                low: 95.0,
                close: 102.0,
                volume: 49.0,
            },
            Bar {
                timestamp: "2023-01-01T00:01:00Z".to_string(),
                open: 102.0,
                high: 106.0,
                low: 98.0,
                close: 104.0,
                volume: 49.0,
            },
            Bar {
                timestamp: "2023-01-01T00:02:00Z".to_string(),
                open: 104.0,
                high: 107.0,
                low: 99.0,
                close: 103.0,
                volume: 49.0,
            },
            Bar {
                timestamp: "2023-01-01T00:03:00Z".to_string(),
                open: 103.0,
                high: 108.0,
                low: 100.0,
                close: 105.0,
                volume: 49.0,
            },
            Bar {
                timestamp: "2023-01-01T00:04:00Z".to_string(),
                open: 103.0,
                high: 107.0,
                low: 100.0,
                close: 106.0,
                volume: 49.0,
            },
        ];

        let result = xavg.calculate_xaverage(&data, 3);
        assert_eq!(result.len(), data.len());
        assert_eq!(result[0], data[0].close);
        assert_eq!(result[1], 103.0);
        assert_eq!(result[2], 103.0);
        assert_eq!(result[3], 104.0);
        assert_eq!(result[4], 105.0);
    }
}
