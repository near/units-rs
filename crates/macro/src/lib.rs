extern crate proc_macro;
use std::num::ParseIntError;

use near_units_core::{gas, near};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ExprLit, Lit};

fn parse_knobs<F>(item: TokenStream, parse: F) -> TokenStream
where
    F: FnOnce(&str) -> Result<u128, ParseIntError>,
{
    match syn::parse::<ExprLit>(item) {
        Ok(ExprLit {
            lit: Lit::Str(str), ..
        }) => {
            let str = parse(&str.value()).unwrap();
            TokenStream::from(quote! {#str})
        }
        _ => TokenStream::from(
            syn::Error::new(
                Span::call_site(),
                "parse can only be used with string literals",
            )
            .to_compile_error(),
        ),
    }
}

#[proc_macro]
pub fn parse_near(item: TokenStream) -> TokenStream {
    parse_knobs(item, near::parse)
}

#[proc_macro]
pub fn parse_gas(item: TokenStream) -> TokenStream {
    parse_knobs(item, |x| gas::parse(x))
}
