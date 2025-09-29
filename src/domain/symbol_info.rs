use serde::{Deserialize, Serialize};

use crate::domain::SymbolType;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum VisiblePlotSet {
    C,
    OHLC,
    OHLCV,
}

/// SymbolInfo
///
/// This struct represents information about a financial symbol.
/// https://www.tradingview.com/charting-library-docs/latest/api/interfaces/Charting_Library.LibrarySymbolInfo/
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SymbolInfo {
    pub name: String,
    pub description: String,
    pub exchange: String,
    pub listed_exchange: String,
    pub minmov: i32,
    pub pricescale: i32,
    pub session: String,
    pub supported_resolutions: String,
    pub timezone: String,
    #[serde(rename(serialize = "type"))]
    pub symbol_type: SymbolType,
    pub build_seconds_from_ticks: Option<bool>,
    pub corrections: Option<String>,
    pub currency_code: Option<String>,
    pub data_status: Option<String>,
    pub delayed: Option<i32>,
    pub exchange_logo: Option<String>,
    pub expiration_date: Option<i32>,
    pub expired: Option<bool>,
    pub format: Option<String>,
    pub fractional: Option<bool>,
    pub has_daily: Option<bool>,
    pub has_empty_bars: Option<bool>,
    pub has_intraday: Option<bool>,
    pub has_seconds: Option<bool>,
    pub has_ticks: Option<bool>,
    pub has_weekly_and_monthly: Option<bool>,
    pub industry: Option<String>,
    pub logo_urls: Option<String>,
    pub long_description: Option<String>,
    pub original_currency_code: Option<String>,
    pub sector: Option<String>,
    pub session_display: Option<String>,
    pub ticker: Option<String>,
    pub unit_id: Option<String>,
    pub variable_tick_size: Option<String>,
    pub visibible_plots_set: Option<VisiblePlotSet>,
    pub volume_precision: Option<i32>,
}
