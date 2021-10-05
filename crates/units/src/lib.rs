pub use near_units_core::*;
pub use near_units_macro::*;

#[cfg(test)]
mod tests {
  use super::*;
  const ONE_NEAR: u128  = parse_near!("1N");
  const ONE_TGAS: u128  = parse_gas!("1 TGas");

    #[test]
    fn const_parse_works() {
      assert_eq!(near::parse("1N").unwrap(), ONE_NEAR);
      assert_eq!(gas::parse("1 TGas").unwrap(), ONE_TGAS)
    }
}
