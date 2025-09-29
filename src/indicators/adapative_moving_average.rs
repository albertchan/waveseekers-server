use std::collections::VecDeque;

use crate::domain::{Bar, Calculation};

pub struct AMA;

/// Adaptive Moving Average (AMA)
///
/// Also known as Kaufman's Adaptive Moving Average, adjusts its smoothing factor
/// based on market volatility and price changes.
///
/// AMA_t = AMA_{t-1} + SmoothingConstant * (price_t - AMA_{t-1})
///
/// where:
/// - AMA_t = current AMA value
/// - AMA_{t-1} = previous AMA value
/// - Smoothing Constant (SC) = (ER * (fastest - slowest) + slowest)^2
/// - Efficiency Ratio (ER) = |price_t - price_{t-n}| / volatility
/// - volatility = sum(abs(Price_t - Price_{t-1}), er_period)
/// - fastest = 2 / (fast_period + 1)
/// - slowest = 2 / (slow_period + 1)
/// - price_t = current price
/// - price_{t-n} = price er_period ago
///
/// The parameters for the AMA calculation are:
/// - er_period: The period for calculating the Efficiency Ratio
/// - fast_period: The period for the fast smoothing factor
/// - slow_period: The period for the slow smoothing factor
///
/// https://corporatefinanceinstitute.com/resources/career-map/sell-side/capital-markets/kaufmans-adaptive-moving-average-kama/
impl AMA {
    pub fn calculate_adaptive_moving_average(
        &self,
        data: &Vec<Bar>,
        er_period: usize,
        fast_period: usize,
        slow_period: usize,
    ) -> VecDeque<f32> {
        let mut ama_data = VecDeque::new();
        let mut price_changes = VecDeque::new();
        let mut prev_prices = VecDeque::new();
        let mut prev_ama = 0.0;
        let mut prev_price = 0.0;
        let mut er_price = 0.0;
        let mut efficiency_ratio;
        let mut volatility;
        let mut smoothing_constant;
        let fastest = 2.0 / (fast_period as f32 + 1.0);
        let slowest = 2.0 / (slow_period as f32 + 1.0);

        for (i, bar) in data.iter().enumerate() {
            if i == 0 {
                er_price = bar.close;
                prev_price = bar.close;
                prev_ama = bar.close;
                price_changes.push_back(0.0);
                prev_prices.push_back(bar.close);
                ama_data.push_back(bar.close);
                continue;
            }

            let price_change = (bar.close - prev_price).abs();
            price_changes.push_back(price_change);
            prev_prices.push_back(bar.close);

            if price_changes.len() > er_period {
                price_changes.pop_front();
                er_price = prev_prices.pop_front().unwrap_or(er_price);
            }

            if price_changes.len() == er_period {
                volatility = price_changes.iter().sum::<f32>();
                efficiency_ratio = if volatility != 0.0 {
                    (bar.close - er_price).abs() / volatility
                } else {
                    0.0
                };
            } else {
                efficiency_ratio = 0.0; // Not enough data yet
            }

            smoothing_constant = (efficiency_ratio * (fastest - slowest) + slowest).powi(2);

            let ama_value = prev_ama + smoothing_constant * (bar.close - prev_ama);
            ama_data.push_back(prev_ama + smoothing_constant * (bar.close - prev_ama));

            prev_ama = ama_value;
            prev_price = bar.close;
        }

        ama_data
    }
}

impl Calculation for AMA {
    fn calculate(&self, data: &Vec<Bar>, params: &Vec<f32>) -> VecDeque<f32> {
        let er_period = params.get(0).cloned().unwrap_or(10.0) as usize;
        let fast_period = params.get(1).cloned().unwrap_or(2.0) as usize;
        let slow_period = params.get(2).cloned().unwrap_or(30.0) as usize;
        self.calculate_adaptive_moving_average(data, er_period, fast_period, slow_period)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Bar;
    use crate::indicators::adapative_moving_average::AMA;

    #[test]
    fn test_calculate_adaptive_moving_average() {
        let ama = AMA {};
        let data = vec![
            Bar {
                timestamp: "2023-01-01T00:00:00Z".parse().unwrap(),
                open: 100.0,
                high: 105.0,
                low: 95.0,
                close: 102.0,
                volume: 49.0,
            },
            Bar {
                timestamp: "2023-01-01T00:01:00Z".parse().unwrap(),
                open: 102.0,
                high: 106.0,
                low: 98.0,
                close: 104.0,
                volume: 49.0,
            },
            Bar {
                timestamp: "2023-01-01T00:02:00Z".parse().unwrap(),
                open: 104.0,
                high: 107.0,
                low: 99.0,
                close: 103.0,
                volume: 49.0,
            },
            Bar {
                timestamp: "2023-01-01T00:03:00Z".parse().unwrap(),
                open: 103.0,
                high: 108.0,
                low: 100.0,
                close: 105.0,
                volume: 49.0,
            },
            Bar {
                timestamp: "2023-01-01T00:04:00Z".parse().unwrap(),
                open: 103.0,
                high: 107.0,
                low: 100.0,
                close: 106.0,
                volume: 49.0,
            },
        ];
        let er_period = 2;
        let fast_period = 2;
        let slow_period = 5;
        let result =
            ama.calculate_adaptive_moving_average(&data, er_period, fast_period, slow_period);

        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 102.0);
        assert_eq!(result[1], 102.888885);
        assert_eq!(result[2], 102.910835);
        assert_eq!(result[3], 103.32351);
        assert_eq!(result[4], 104.51306);
    }
}
