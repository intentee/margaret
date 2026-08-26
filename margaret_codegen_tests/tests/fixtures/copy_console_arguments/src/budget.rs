use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct Budget(pub u32);

impl FromStr for Budget {
    type Err = ParseIntError;

    fn from_str(value: &str) -> Result<Self, ParseIntError> {
        value.parse().map(Self)
    }
}
