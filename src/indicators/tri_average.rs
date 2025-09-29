use std::collections::VecDeque;

use crate::domain::{Bar, Calculation};

pub struct TMA;

/// TriAverage or Triangular Moving Average (TMA)
///
/// TMA is a double smoothed simple moving average that is used to reduce noise in price data.
///
/// var0 = Ceil(( Len + 1 ) * .5);
/// TMA = Average( Average( PriceValue, var0 ), var0 );
impl TMA {
    pub fn calculate_tri_average(&self, data: &Vec<Bar>, period: usize) -> VecDeque<f32> {
        // let len = (period + 1).div_ceil(2);
        let mut tma_data: VecDeque<f32> = VecDeque::new();
        let mut avg1_data: VecDeque<f32> = VecDeque::new();
        let mut sum1 = 0.0;
        let mut sum2 = 0.0;

        for (i, bar) in data.iter().enumerate() {
            sum1 += bar.close;

            if i >= period - 1 {
                if i >= period {
                    sum1 -= data[i - period].close; // Remove the oldest value from the sum
                }
                let avg1 = sum1 / period as f32;
                avg1_data.push_back(avg1);
                sum2 += avg1;

                if i >= 2 * period - 2 {
                    if i >= 2 * period - 1 {
                        if let Some(val) = avg1_data.pop_front() {
                            sum2 -= val; // Remove the oldest value from the sum
                        }
                    }
                    tma_data.push_back(sum2 / period as f32);
                } else {
                    tma_data.push_back(0.0); // Not enough data yet
                }
            } else {
                tma_data.push_back(0.0); // Not enough data yet
            }
        }

        tma_data
    }
}

impl Calculation for TMA {
    fn calculate(&self, data: &Vec<Bar>, params: &Vec<f32>) -> VecDeque<f32> {
        let period = params.get(0).cloned().unwrap_or(14.0) as usize;
        self.calculate_tri_average(data, period)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Bar;
    use crate::indicators::tri_average::TMA;

    #[test]
    fn test_calculate_tri_average() {
        let tma = TMA;
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

        let result = tma.calculate_tri_average(&data, 2);
        assert_eq!(result.len(), data.len());
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 0.0);
        assert_eq!(result[2], 103.25);
        assert_eq!(result[3], 103.75);
        assert_eq!(result[4], 104.75);

        let result = tma.calculate_tri_average(&data, 3);
        assert_eq!(result.len(), data.len());
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 0.0);
        assert_eq!(result[2], 0.0);
        assert_eq!(result[3], 0.0);
        assert_eq!(result[4], 103.888889);
    }
}
