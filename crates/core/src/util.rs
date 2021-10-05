use crate::prefixes::{from_magnitude, MAGNITUDES, PATTERNS};
use num_format::{Locale, ToFormattedString};
use regex::{Regex, RegexSet};

pub fn get_match(s: &str) -> Option<usize> {
    let set = RegexSet::new(&PATTERNS).unwrap();
    let matches: Vec<usize> = set.matches(s.trim_end()).into_iter().collect();
    if matches.len() != 1 {
        return None;
    }
    matches.get(0).copied()
}

pub fn get_magnitude(s: &str) -> i8 {
    get_match(s).map_or(0, |m| MAGNITUDES[m])
}

pub fn clean(x: &str) -> String {
    Regex::new(r"(?:[,_])|^(0|\s)+\b|(?i:\s|[μa-z])+$")
        .unwrap()
        .replace_all(x, "")
        .to_string()
}

pub fn parse(with_units: &str, magnitude: i8) -> Option<String> {
    let maginitude = magnitude + get_magnitude(with_units);
    let num_str = clean(with_units);
    let mut parts = num_str.split('.');
    let whole_part = parts.next().unwrap();
    let fraction_part = parts.next().unwrap_or("");

    // For now these are options but will be refactor into errors
    // Means more than one `.`s
    if parts.next().is_some() {
        return None;
    }
    if maginitude == 0 && !fraction_part.is_empty() {
        return None;
    }
    if fraction_part.len() as i8 > maginitude {
        return None;
    }

    return Some(format!(
        "{}{}{}",
        whole_part,
        fraction_part,
        "0".repeat(maginitude as usize - fraction_part.len())
    ));
}

pub fn to_human(num: u128, base_unit: &str, maginitude: i8, adjust_magnitude: i8) -> String {
    let nomination = u128::pow(10, maginitude as u32);
    let quotient = num / nomination;
    let remainder = num % nomination;

    if quotient > 0 {
        let int = quotient.to_formatted_string(&Locale::en);
        let remainder_str = remainder.to_string();
        let fraction = if remainder == 0 {
            "".to_string()
        } else {
            let pad = (maginitude as isize - remainder_str.len() as isize).max(0) as usize;
            format!(".{}{}", "0".repeat(pad), remainder_str)
                .trim_end_matches('0')
                .to_string()
        };
        let prefix = from_magnitude(adjust_magnitude).unwrap();
        return format!("{}{} {}{}", int, fraction, prefix, base_unit);
    }

    to_human(num, base_unit, maginitude - 3, adjust_magnitude - 3)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(get_match("y").unwrap(), 19);
        assert_eq!(get_magnitude("y"), -24);
        assert_eq!(get_match("yocto").unwrap(), 19);
        assert_eq!(get_magnitude("yocto"), -24);
        assert_eq!(get_magnitude("1yocto"), -24);
    }

    #[test]
    fn parse_test() {
        assert_eq!(parse("1000m", 24).unwrap(), "1000000000000000000000000");
    }
}
