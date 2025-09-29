use actix_web::{web, HttpResponse, Responder};

use crate::{domain::{SymbolInfo, SymbolType}, routes::symbols::search};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolsQueryParams {
    name: Option<String>,
    exchange: Option<String>,
    symbol_type: Option<SymbolType>,
}

pub async fn symbols_handler(
    query: web::Query<SymbolsQueryParams>,
) -> impl Responder {
    if let Some(name) = &query.name {
        match search(&name, &query.exchange, &query.symbol_type) {
            Ok(val) => {
                if val.is_empty() {
                    HttpResponse::NotFound().body("Symbol not found")
                } else {
                    HttpResponse::Ok().json(val)
                }
            },
            Err(err) => {
                HttpResponse::InternalServerError().body(format!("error resolving symbol: {}", err))
            }
        }
    } else {
        match fetch_symbols() {
            Ok(val) => HttpResponse::Ok().json(val),
            Err(err) => {
                HttpResponse::InternalServerError().body(format!("error fetching symbols: {}", err))
            }
        }
    }
}

pub async fn get_all_symbols() -> impl Responder {
    match fetch_symbols() {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("error fetching symbols: {}", err))
        }
    }
}

pub fn fetch_symbols() -> Result<Vec<SymbolInfo>, csv::Error> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let res_directory = base_path.join("data");
    let filename = res_directory.join("symbols.csv");

    let reader = csv::Reader::from_path(filename);
    let mut symbols: Vec<SymbolInfo> = Vec::new();

    for result in reader?.deserialize() {
        let record: SymbolInfo = result?;
        symbols.push(record);
    }

    Ok(symbols)
}
