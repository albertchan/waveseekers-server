use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Interval {
    T49 = 49,
    T343 = 343,
    T2401 = 2401,
    T16807 = 16807,
    T117649 = 117649,
    T823543 = 823543,
    T5764801 = 5764801,
    T40353607 = 40353607,
}

impl Interval {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}
