use chrono::NaiveDateTime;

use crate::datafeed::generate_from_ticks;
use crate::domain::{Bar, EnhancedSeries, Indicators, Interval, Resolution, Series, Tick};
use crate::storage::{Database, Error, QueryResult};

#[derive(Clone)]
pub struct HistoricalData {
    database: Database,
}

impl HistoricalData {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn fetch(
        &self,
        symbol: &String,
        exchange: &String,
        resolution: &Resolution,
        interval: &Interval,
        from: &NaiveDateTime,
        to: &Option<NaiveDateTime>,
        limit_lower: &Option<usize>,
        limit_upper: &Option<usize>,
        indicators: &Option<Vec<Indicators>>,
    ) -> Result<EnhancedSeries, Error> {
        match resolution {
            Resolution::TICK => {
                let from_str = from.format("%Y-%m-%d %H:%M:%S").to_string();
                let to_str = to
                    .as_ref()
                    .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string());

                fetch_enhanced_series(
                    &self.database,
                    symbol,
                    exchange,
                    resolution,
                    interval,
                    &from_str,
                    &to_str,
                    limit_lower,
                    limit_upper,
                    indicators,
                )
                .await
            }
            _ => Err(Error::UnsupportedResolution),
        }
    }
}

pub async fn fetch_enhanced_series<'a>(
    database: &'a Database,
    symbol: &'a String,
    exchange: &'a String,
    resolution: &'a Resolution,
    interval: &'a Interval,
    from: &'a String,
    to: &'a Option<String>,
    limit_lower: &'a Option<usize>,
    limit_upper: &'a Option<usize>,
    indicators: &'a Option<Vec<Indicators>>,
) -> Result<EnhancedSeries, Error> {
    let mut result: QueryResult<Tick> = fetch_ticks(
        &database,
        &symbol,
        &exchange,
        &from,
        &to,
        &limit_lower,
        &limit_upper,
    )
    .await
    .expect("Failed to fetch ticks");

    if result.data.is_empty() {
        return Err(Error::NoDataFound);
    }

    let series = Series::new(
        symbol.clone(),
        resolution.clone(),
        interval.clone(),
        generate_from_ticks(&mut result.data, &interval),
    )
    .enhance_with(indicators.as_ref().unwrap_or(&vec![]));

    Ok(series)
}

pub async fn fetch_bars(
    database: &Database,
    symbol: &String,
    exchange: &String,
    interval: &Interval,
    from: &String,
    to: &Option<String>,
    limit_lower: &Option<usize>,
    limit_upper: &Option<usize>,
) -> Result<QueryResult<Bar>, Error> {
    let mut result: QueryResult<Tick> = fetch_ticks(
        &database,
        &symbol,
        &exchange,
        &from,
        &to,
        &limit_lower,
        &limit_upper,
    )
    .await
    .expect("Failed to fetch ticks");

    if result.data.is_empty() {
        return Err(Error::NoDataFound);
    }

    Ok(QueryResult {
        data: generate_from_ticks(&mut result.data, &interval),
    })
}

pub async fn fetch_ticks(
    database: &Database,
    symbol: &String,
    exchange: &String,
    from: &String,
    to: &Option<String>,
    limit_lower: &Option<usize>,
    limit_upper: &Option<usize>,
) -> Result<QueryResult<Tick>, questdb::Error> {
    let table_name = get_table_name(exchange, &Resolution::TICK);
    let mut query = format!(
        "SELECT * FROM {} WHERE symbol = '{}' AND timestamp >= '{}'",
        table_name,
        symbol.to_uppercase(),
        from
    );

    if let Some(t) = to {
        query += format!(" AND timestamp <= '{}'", t).as_str();
    }

    Ok(database
        .exec::<Tick>(query.as_str(), limit_lower, limit_upper)
        .await
        .unwrap())
}

fn get_table_name(exchange: &String, resolution: &Resolution) -> String {
    let mut table_name = exchange.to_lowercase();
    let resolution = resolution.to_string().to_lowercase();
    table_name.push_str("_");
    table_name.push_str(&resolution);
    table_name
}
