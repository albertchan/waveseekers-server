use std::collections::VecDeque;

use crate::domain::{Bar, Calculation};

pub struct SMA;

impl SMA {
    pub fn calculate_simple_moving_average(&self, data: &Vec<Bar>, period: usize) -> VecDeque<f32> {
        let mut sma_data = VecDeque::new();
        let mut sum = 0.0;

        for (i, bar) in data.iter().enumerate() {
            sum += bar.close;

            if i >= period - 1 {
                if i >= period {
                    sum -= data[i - period].close; // Remove the oldest value from the sum
                }
                sma_data.push_back(sum / period as f32);
            } else {
                sma_data.push_back(0.0); // Not enough data yet
            }
        }

        sma_data
    }
}

impl Calculation for SMA {
    fn calculate(&self, data: &Vec<Bar>, params: &Vec<f32>) -> VecDeque<f32> {
        let period = params.get(0).cloned().unwrap_or(14.0) as usize;
        self.calculate_simple_moving_average(data, period)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Bar;
    use crate::indicators::simple_moving_average::SMA;

    #[test]
    fn test_calculate_simple_moving_average() {
        let sma = SMA;
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

        let result = sma.calculate_simple_moving_average(&data, 2);
        assert_eq!(result.len(), data.len());
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], (102.0 + 104.0) / 2.0); // 103.0
        assert_eq!(result[2], (104.0 + 103.0) / 2.0); // 103.5
        assert_eq!(result[3], (103.0 + 105.0) / 2.0); // 104.0
        assert_eq!(result[4], (105.0 + 106.0) / 2.0); // 105.5
    }
}
