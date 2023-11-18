#![warn(clippy::pedantic)]

use proc_macro::TokenStream;

#[proc_macro_derive(EnumSelect)]
pub fn derive_enum_select(input: TokenStream) -> TokenStream {
    seenum_core::enum_select::derive(input.into()).into()
}

#[proc_macro_derive(Display, attributes(display))]
pub fn derive_display(input: TokenStream) -> TokenStream {
    seenum_core::display::derive(input.into()).into()
}
