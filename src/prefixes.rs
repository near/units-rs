pub static MAGNITUDES: phf::Map<&'static str, i8> = phf::phf_map! {
  r"(?:yotta|Y)\s*$" => 24,
  r"(?:zetta|Z)\s*$" => 21,
  r"(?:exa|E)\s*$" => 18,
  r"(?:peta|P)\s*$" => 15,
  r"(?:terra|T)\s*$" => 12,
  r"(?:giga|G)\s*$" => 9,
  r"(?:mega|M)\s*$" => 6,
  r"(?:kilo|k)\s*$" => 3,
  r"(?:hecto|h)\s*$" => 2,
  r"(?:deka|da)\s*$" => 1,
  r"(?:deci|d)\s*$" => -1,
  r"(?:centi|c)\s*$" => -2,
  r"(?:milli|m)\s*$" => -3,
  r"(?:micro|μ)\s*$" => -6,
  r"(?:nano|n)\s*$" => -9,
  r"(?:pico|p)\s*$" => -12,
  r"(?:femto|f)\s*$" => -15,
  r"(?:atto|a)\s*$" => -18,
  r"(?:zepto|z)\s*$" => -21,
  r"(?:yocto|y)\s*$" => -24
};

pub const YOTTA: &str = r"(?:yotta|Y)\s*$";
pub const ZETTA: &str = r"(?:zetta|Z)\s*$";
pub const EXA: &str = r"(?:exa|E)\s*$";
pub const PETA: &str = r"(?:peta|P)\s*$";
pub const TERRA: &str = r"(?:terra|T)\s*$";
pub const GIGA: &str = r"(?:giga|G)\s*$";
pub const MEGA: &str = r"(?:mega|M)\s*$";
pub const KILO: &str = r"(?:kilo|k)\s*$";
pub const HECTO: &str = r"(?:hecto|h)\s*$";
pub const DEKA: &str = r"(?:deka|da)\s*$";
pub const DECI: &str = r"(?:deci|d)\s*$";
pub const CENTI: &str = r"(?:centi|c)\s*$";
pub const MILLI: &str = r"(?:milli|m)\s*$";
pub const MICRO: &str = r"(?:micro|μ)\s*$";
pub const NANO: &str = r"(?:nano|n)\s*$";
pub const PICO: &str = r"(?:pico|p)\s*$";
pub const FEMTO: &str = r"(?:femto|f)\s*$";
pub const ATTO: &str = r"(?:atto|a)\s*$";
pub const ZEPTO: &str = r"(?:zepto|z)\s*$";
pub const YOCTO: &str = r"(?:yocto|y)\s*$";

pub const PATTERNS: [&str; 20] = [
    YOTTA, ZETTA, EXA, PETA, TERRA, GIGA, MEGA, KILO, HECTO, DEKA, DECI, CENTI, MILLI, MICRO, NANO,
    PICO, FEMTO, ATTO, ZEPTO, YOCTO,
];

pub fn from_magnitude(magnitude: i8) -> Option<&'static str> {
    match magnitude {
        24 => Some("Y"),
        21 => Some("Z"),
        18 => Some("E"),
        15 => Some("P"),
        12 => Some("T"),
        9 => Some("G"),
        6 => Some("M"),
        3 => Some("k"),
        0 => Some(""),
        -3 => Some("m"),
        -6 => Some("μ"),
        -9 => Some("n"),
        -12 => Some("p"),
        -15 => Some("f"),
        -18 => Some("a"),
        -21 => Some("z"),
        -24 => Some("y"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lookup_works() {
        assert_eq!(MAGNITUDES[YOCTO], -24);
        assert_eq!(MAGNITUDES[YOTTA], 24)
    }
}
