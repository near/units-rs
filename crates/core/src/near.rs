use std::num::ParseIntError;

use regex::Regex;

pub fn parse_str(input: &str) -> Option<String> {
    let near = Regex::new(r"(?i:n(?i:ear)?)\s*$")
        .unwrap()
        .replace_all(input, "")
        .to_string();
    return crate::util::parse(&near, 24);
}

pub fn parse(input: &str) -> Result<u128, ParseIntError> {
    let int_str = parse_str(input).expect("Cannot parse string");
    u128::from_str_radix(&int_str, 10)
}

pub fn to_human(input: u128) -> String {
    crate::util::to_human(input, "N", 24, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: [[&str; 3]; 26] = [
        ["1", "1000000000000000000000000", "1 N"],
        [".1000000000000", "100000000000000000000000", "100 mN"],
        ["1,000", "1000000000000000000000000000", "1,000 N"],
        ["1.0", "1000000000000000000000000", "1 N"],
        [
            "1,000,000",
            "1000000000000000000000000000000",
            "1,000,000 N",
        ],
        [
            "1,000,000.000_000_01   ",
            "1000000000000010000000000000000",
            "1,000,000.00000001 N",
        ],
        ["1MN", "1000000000000000000000000000000", "1,000,000 N"],
        ["1kN   ", "1000000000000000000000000000", "1,000 N"],
        ["0.001_101", "1101000000000000000000", "1.101 mN"],
        ["0.000,101", "101000000000000000000", "101 μN"],
        ["1mN", "1000000000000000000000", "1 mN"],
        ["1 milliN", "1000000000000000000000", "1 mN"],
        [" 001      m N    ", "1000000000000000000000", "1 mN"],
        ["1 milliNEAR", "1000000000000000000000", "1 mN"],
        ["1 milliN", "1000000000000000000000", "1 mN"],
        ["1 millinear", "1000000000000000000000", "1 mN"],
        ["1 milli   ", "1000000000000000000000", "1 mN"],
        ["1 m", "1000000000000000000000", "1 mN"],
        ["1μ", "1000000000000000000", "1 μN"],
        ["1micro", "1000000000000000000", "1 μN"],
        ["1nN", "1000000000000000", "1 nN"],
        ["1p", "1000000000000", "1 pN"],
        ["1f", "1000000000", "1 fN"],
        ["1a", "1000000", "1 aN"],
        ["1z", "1000", "1 zN"],
        ["1y", "1", "1 yN"],
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
