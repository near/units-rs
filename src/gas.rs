use std::num::ParseIntError;
use regex::Regex;

pub fn parse(from: &str) -> Option<String> {
    let gas = Regex::new(r"(?:gas)\s*").unwrap().replace_all(from, "").to_string();
    return crate::util::parse(&gas, 0);
}


pub fn parse_u128(input: &str) -> Result<u128, ParseIntError>  {
  let int_str = parse(input).expect("Cannot parse string");
  u128::from_str_radix(&int_str, 10)
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
            let parsed = parse_u128(line[0]).unwrap();
            let expected = line[1];
            assert_eq!(parsed.to_string(), expected);
        }
    }
}
