extern crate proc_macro;
use near_units_core::{gas, near};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ExprLit, Lit};

#[proc_macro]
pub fn parse_near(item: TokenStream) -> TokenStream {
    match syn::parse::<ExprLit>(item) {
        Ok(ExprLit {
            lit: Lit::Str(str), ..
        }) => {
            let str = near::parse(&str.value()).unwrap();
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
pub fn parse_gas(item: TokenStream) -> TokenStream {
    match syn::parse::<ExprLit>(item) {
        Ok(ExprLit {
            lit: Lit::Str(str), ..
        }) => {
            let str = gas::parse(&str.value()).unwrap();
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
