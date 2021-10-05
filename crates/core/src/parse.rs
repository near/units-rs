
use std::num::ParseIntError;

use crate::near;
use crate::gas;

pub fn parse(input: &str) -> Result<u128, ParseIntError>  {
  near::parse(input).or_else( |_| gas::parse(input))
}