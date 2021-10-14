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
pub static MAGNITUDES: [i8; 20] = [
    24, 21, 18, 15, 12, 9, 6, 3, 2, 1, -1, -2, -3, -6, -9, -12, -15, -18, -21, -24,
];

pub const fn from_magnitude(magnitude: i8) -> Option<&'static str> {
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
