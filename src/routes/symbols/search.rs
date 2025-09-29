use crate::{domain::{SymbolInfo, SymbolType}, routes::fetch_symbols};

pub fn search(
    name: &String, 
    exchange: &Option<String>,
    symbol_type: &Option<SymbolType>
) -> Result<Vec<SymbolInfo>, anyhow::Error> {
    let mut symbols: Vec<SymbolInfo> = Vec::new();

    match fetch_symbols() {
        Ok(val) => {
            for symbol in val {
                if symbol.name.to_lowercase().contains(&name.to_lowercase()) {
                    if let Some(exchange) = exchange {
                        if &symbol.exchange != exchange {
                            continue;
                        }
                    }
                    if let Some(symbol_type) = symbol_type {
                        if &symbol.symbol_type != symbol_type {
                            continue;
                        }
                    }
                    symbols.push(symbol);
                }
            }
        },
        Err(err) => {
            return Err(anyhow::anyhow!(err));
        }
    };

    Ok(symbols)
}
