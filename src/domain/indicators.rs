use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::vec;

use crate::domain::Bar;
use crate::indicators::{AMA, SMA, TMA, XAverage};

pub trait Calculation {
    fn calculate(&self, data: &Vec<Bar>, params: &Vec<f32>) -> VecDeque<f32>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Indicator {
    pub name: String,
    pub params: Vec<f32>,
    pub data: VecDeque<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Indicators {
    AMA(usize, usize, usize), // er_period, fast_period, slow_period
    SMA(usize),
    TMA(usize),
    XAverage(usize),
    Unknown(String),
}

impl Indicators {
    pub fn name(&self) -> &str {
        match self {
            Indicators::AMA(_, _, _) => "ama",
            Indicators::SMA(_) => "sma",
            Indicators::TMA(_) => "tma",
            Indicators::XAverage(_) => "xaverage",
            Indicators::Unknown(name) => name,
        }
    }

    pub fn params(&self) -> Vec<f32> {
        match self {
            Indicators::AMA(er_period, fast_period, slow_period) => {
                vec![*er_period as f32, *fast_period as f32, *slow_period as f32]
            }
            Indicators::SMA(period) => vec![*period as f32],
            Indicators::TMA(period) => vec![*period as f32],
            Indicators::XAverage(period) => vec![*period as f32],
            Indicators::Unknown(_) => vec![],
        }
    }
}

impl From<(&str, &Vec<f32>)> for Indicators {
    fn from((name, params): (&str, &Vec<f32>)) -> Self {
        match name {
            "ama" => {
                let er_period = params.get(0).cloned().unwrap_or(10.0) as usize;
                let fast_period = params.get(1).cloned().unwrap_or(2.0) as usize;
                let slow_period = params.get(2).cloned().unwrap_or(30.0) as usize;
                Indicators::AMA(er_period, fast_period, slow_period)
            }
            "sma" => Indicators::SMA(params.get(0).cloned().unwrap_or(14.0) as usize),
            "tma" => Indicators::TMA(params.get(0).cloned().unwrap_or(9.0) as usize),
            "xaverage" => Indicators::XAverage(params.get(0).cloned().unwrap_or(9.0) as usize),
            // Add more indicators here as needed
            _ => Indicators::Unknown(name.to_string()),
        }
    }
}

pub fn calculate(name: &str, data: &Vec<Bar>, params: &Vec<f32>) -> VecDeque<f32> {
    match name {
        "ama" => AMA.calculate(data, params),
        "sma" => SMA.calculate(data, params),
        "tma" => TMA.calculate(data, params),
        "xaverage" => XAverage.calculate(data, params),
        // Add more indicators as needed
        _ => VecDeque::new(),
    }
}
