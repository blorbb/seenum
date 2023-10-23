#![warn(clippy::pedantic)]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_derive(EnumSelect)]
#[proc_macro_error]
pub fn derive_seenum(input: TokenStream) -> TokenStream {
    seenum_core::derive_enum_select(input.into()).into()
}

#[proc_macro_derive(Display, attributes(display))]
#[proc_macro_error]
pub fn derive_display(input: TokenStream) -> TokenStream {
    match seenum_core::derive_display(input.into()) {
        Ok(o) => o.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
