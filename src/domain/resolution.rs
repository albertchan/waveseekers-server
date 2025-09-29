use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Resolution {
    TICK,
    MINUTE,
    HOUR,
    DAY,
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Resolution::TICK => write!(f, "{}", "TICK"),
            Resolution::MINUTE => write!(f, "{}", "MINUTE"),
            Resolution::HOUR => write!(f, "{}", "HOUR"),
            Resolution::DAY => write!(f, "{}", "DAY"),
        }
    }
}
