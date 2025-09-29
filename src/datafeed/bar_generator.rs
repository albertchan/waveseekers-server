use actix_web::web;

use crate::{
    datafeed::fetch_ticks,
    domain::{Bar, Interval, Tick},
    storage::{Database, QueryResult},
};

pub fn generate_from_ticks(mut ticks: &mut Vec<Tick>, interval: &Interval) -> Vec<Bar> {
    let mut bars = Vec::new();
    let bar_interval = interval.as_usize() as f32;

    if ticks.is_empty() {
        return bars;
    }

    let first_tick = get_first_tick(&mut ticks);
    let first_price = first_tick.as_ref().map_or(0.0, |tick| tick.price);
    let mut open = first_price;
    let mut high = first_price;
    let mut low = first_price;
    let mut count = first_tick.as_ref().map_or(0.0, |tick| tick.volume);

    // We skip the first tick since it is already used to set open, high, low
    for tick in ticks.iter_mut().skip(1) {
        // keep count of volume
        count = count + tick.volume;

        while count >= bar_interval {
            count = count - bar_interval;
            let bar = Bar {
                timestamp: tick.timestamp.clone(),
                open,
                high,
                low,
                close: tick.price,
                volume: bar_interval,
            };
            bars.push(bar);

            // reset values
            open = tick.price;
            high = tick.price;
            low = tick.price;
        }
    }

    bars
}

pub async fn generate_bars(
    database: web::Data<Database>,
    symbol: &String,
    exchange: &String,
    interval: &Interval,
    from: &String,
    to: &Option<String>,
) -> Result<Vec<Bar>, anyhow::Error> {
    let mut bars = Vec::new();
    let mut result: QueryResult<Tick> =
        fetch_ticks(&database, &symbol, &exchange, &from, &to, &None, &None)
            .await
            .expect("Failed to generate bars");

    // TODO - handle case where no ticks are returned
    // TODO - remove multiple ticks with same timestamp
    // TODO - handle case where ticks are not in order
    if result.data.is_empty() {
        return Ok(bars);
    }

    let bar_interval = interval.as_usize() as f32;
    let first_tick = get_first_tick(&mut result.data);
    let first_price = first_tick.as_ref().map_or(0.0, |tick| tick.price);
    let mut open = first_price;
    let mut high = first_price;
    let mut low = first_price;
    let mut count = first_tick.as_ref().map_or(0.0, |tick| tick.volume);

    // We skip the first tick since it is already used to set open, high, low
    for tick in result.data.iter_mut().skip(1) {
        // keep count of volume
        count = count + tick.volume;

        while count >= bar_interval {
            count = count - bar_interval;
            let bar = Bar {
                timestamp: tick.timestamp.clone(),
                open,
                high,
                low,
                close: tick.price,
                volume: bar_interval,
            };
            bars.push(bar);

            // reset values
            open = tick.price;
            high = tick.price;
            low = tick.price;
        }
    }

    Ok(bars)
}

fn get_first_tick(ticks: &mut Vec<Tick>) -> Option<Tick> {
    if ticks.is_empty() {
        return None;
    }
    Some(ticks.first().cloned().unwrap())
}
