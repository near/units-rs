use std::num::ParseIntError;

pub trait ParseableUnit {
    fn parse(input: &str) -> Option<String>;

    fn parse_u128(input: &str) -> Result<u128, ParseIntError>;

    fn to_human(input: &u128) -> String;
}
