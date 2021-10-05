use regex::Regex;
use std::num::ParseIntError;

pub fn parse_str(from: &str) -> Option<String> {
    let gas = Regex::new(r"(?:gas)\s*")
        .unwrap()
        .replace_all(from, "")
        .to_string();
    return crate::util::parse(&gas, 0);
}

pub fn parse(input: &str) -> Result<u128, ParseIntError> {
    let int_str = parse_str(input).expect("Cannot parse string");
    u128::from_str_radix(&int_str, 10)
}

pub fn to_human(input: u128) -> String {
    crate::util::to_human(input, "gas", 12, 12)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: [[&str; 3]; 7] = [
        ["1", "1", "1 gas"],
        ["1,000", "1000", "1 kgas"],
        ["1,000,000", "1000000", "1 Mgas"],
        ["1,000,000,000", "1000000000", "1 Ggas"],
        ["1,000,000,000,000", "1000000000000", "1 Tgas"],
        ["1Tgas", "1000000000000", "1 Tgas"],
        ["1Ggas", "1000000000", "1 Ggas"],
    ];

    #[test]
    fn it_works() {
        for line in &DATA {
            let parsed = parse(line[0]).unwrap();
            let expected = line[1];
            assert_eq!(parsed.to_string(), expected);
            assert_eq!(to_human(parsed), line[2]);
        }
    }
}
