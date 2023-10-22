use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_derive(EnumSelect)]
#[proc_macro_error]
pub fn derive_enum_select(input: TokenStream) -> TokenStream {
    enum_select_core::derive_enum_select(input.into()).into()
}
