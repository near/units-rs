pub static MAGNITUDES: phf::Map<&'static str, i8> = phf::phf_map! {
  r"yotta|Y$" => 24,
  r"zetta|Z$" => 21,
  r"exa|E$" => 18,
  r"peta|P$" => 15,
  r"terra|T$" => 12,
  r"giga|G$" => 9,
  r"mega|M$" => 6,
  r"kilo|k$" => 3,
  r"hecto|h$" => 2,
  r"deka|da$" => 1,
  r"deci|d$" => -1,
  r"centi|c$" => -2,
  r"milli|m$" => -3,
  r"micro|μ$" => -6,
  r"nano|n$" => -9,
  r"pico|p$" => -12,
  r"femto|f$" => -15,
  r"atto|a$" => -18,
  r"zepto|z$" => -21,
  r"yocto|y$" => -24
};

pub const YOTTA: &str = r"yotta|Y$";
pub const ZETTA: &str = r"zetta|Z$";
pub const EXA: &str = r"exa|E$";
pub const PETA: &str = r"peta|P$";
pub const TERRA: &str = r"terra|T$";
pub const GIGA: &str = r"giga|G$";
pub const MEGA: &str = r"mega|M$";
pub const KILO: &str = r"kilo|k$";
pub const HECTO: &str = r"hecto|h$";
pub const DEKA: &str = r"deka|da$";
pub const DECI: &str = r"deci|d$";
pub const CENTI: &str = r"centi|c$";
pub const MILLI: &str = r"milli|m$";
pub const MICRO: &str = r"micro|μ$";
pub const NANO: &str = r"nano|n$";
pub const PICO: &str = r"pico|p$";
pub const FEMTO: &str = r"femto|f$";
pub const ATTO: &str = r"atto|a$";
pub const ZEPTO: &str = r"zepto|z$";
pub const YOCTO: &str = r"yocto|y$";

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
