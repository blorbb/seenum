use proc_macro2::{Span, TokenStream};
use quote::quote;

pub fn derive(input: TokenStream) -> TokenStream {
    derive_impl(input).unwrap_or_else(syn::Error::into_compile_error)
}

fn derive_impl(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let syn::Data::Enum(data_enum) = input.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "`#[derive(Display)]` is only supported on enums",
        ));
    };

    // (variant name, #[display(...)] contents)
    let pairs: Vec<(proc_macro2::Ident, TokenStream)> = data_enum
        .variants
        .iter()
        .map(|variant| {
            let display_attr = variant
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("display"))
                .ok_or(syn::Error::new_spanned(
                    variant,
                    "`#[display(...)]` attribute required for all variants",
                ))?;
            let inner: TokenStream = display_attr.parse_args()?;

            Ok((variant.ident.clone(), inner))
        })
        .collect::<syn::Result<_>>()?;

    let (name, inner): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    let enum_name = input.ident;
    Ok(quote! {
        impl ::core::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    #(Self::#name => ::core::write!(f, #inner),)*
                }
            }
        }
    })
}
