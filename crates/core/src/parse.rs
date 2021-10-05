use std::num::ParseIntError;

use crate::gas;
use crate::near;

pub fn parse(input: &str) -> Result<u128, ParseIntError> {
    near::parse(input).or_else(|_| gas::parse(input))
}
