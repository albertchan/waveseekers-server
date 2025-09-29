use std::collections::HashMap;

use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

use crate::{
    datafeed::HistoricalData,
    domain::{Indicators, Interval, Resolution},
};

#[derive(Debug, serde::Deserialize)]
pub struct Parameters {
    symbol: String,
    exchange: String,
    resolution: Resolution,
    interval: Interval,
    from: String,
    to: Option<String>,
    limit_lower: Option<usize>,
    limit_upper: Option<usize>,
    #[serde(deserialize_with = "deserialize_stringified_map")]
    indicators: Option<Vec<Indicators>>,
}

pub async fn get_historical_data(
    historical: web::Data<HistoricalData>,
    params: web::Query<Parameters>,
) -> impl Responder {
    historical
        .fetch(
            &params.symbol,
            &params.exchange,
            &params.resolution,
            &params.interval,
            &params.from.parse().unwrap(), // TODO - handle errors (remove unwrap)
            &params.to.as_ref().map(|t| t.parse().unwrap()), // TODO - handle errors (remove unwrap)
            &params.limit_lower,
            &params.limit_upper,
            &params.indicators,
        )
        .await
        .map(|data| HttpResponse::Ok().json(data))
        .unwrap_or_else(|err| {
            HttpResponse::InternalServerError().body(format!("get_historical_data: {}", err))
        })
}

fn deserialize_stringified_map<'de, D>(deserializer: D) -> Result<Option<Vec<Indicators>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            let map: HashMap<String, Vec<f32>> =
                serde_json::from_str(&s).map_err(serde::de::Error::custom)?;
            let indicators: Vec<Indicators> = map
                .into_iter()
                .map(|(name, params)| Indicators::from((name.as_str(), &params)))
                .collect();
            Ok(Some(indicators))
        }
        None => Ok(None),
    }
}
