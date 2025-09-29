use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum SymbolType {
    BOND,
    CFD,
    COMMODITY,
    CRYPTO,
    DR,
    ECONOMIC,
    EQUITY,
    FOREX,
    FUND,
    FUNDAMENTAL,
    FUTURES,
    INDEX,
    OPTION,
    RIGHT,
    SPOT,
    SPREAD,
    STOCK,
    STRUCTURED,
    SWAP,
    WARRANT,
}

impl fmt::Display for SymbolType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
