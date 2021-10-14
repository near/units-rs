pub use near_units_core::*;
pub use near_units_macro::*;


#[cfg(test)]
mod tests {
    use super::*;
    const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
    const ONE_TGAS: u128 = 1_000_000_000_000;
    const ONE_PARSED_NEAR: u128 = parse_near!("1 N");
    const ONE_PARSED_TGAS: u128 = parse_gas! ("1 TGas");

    #[test]
    fn const_parse_near() {
        assert_eq!(near::parse("1N").unwrap(), ONE_NEAR);
        assert_eq!(ONE_PARSED_NEAR, ONE_NEAR);

    }

    #[test]
    fn const_parse_gas() {
      assert_eq!(gas::parse("1 TGas").unwrap(), ONE_TGAS);
      assert_eq!(ONE_PARSED_TGAS, ONE_TGAS);
    }
}
