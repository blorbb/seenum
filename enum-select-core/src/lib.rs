use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, Token};

pub fn derive_enum_select(input: TokenStream) -> TokenStream {
    let input = syn::parse2(input).unwrap_or_else(|e| abort!(e.span(), e.to_string()));
    let UnitEnum { name, variants } = validate_input(input);

    let count = variants.len();
    quote! {
        unsafe impl ::enum_select::EnumSelect for #name {
            const COUNT: ::std::num::NonZeroUsize =
                unsafe { ::std::num::NonZeroUsize::new_unchecked(#count) };

            unsafe fn from_index_unchecked(index: ::std::primitive::usize) -> Self {
                unsafe { ::std::mem::transmute(index) }
            }
        }
    }
}

/// Validates the derive input to be a `#[repr(usize)]` enum with only unit
/// variants and no custom discriminants.
fn validate_input(input: syn::DeriveInput) -> UnitEnum {
    let is_repr_usize = input.attrs.iter().any(|attr| {
        attr.path().is_ident("repr")
            && attr
                .parse_args_with(Punctuated::<syn::Ident, Token![,]>::parse_terminated)
                .unwrap_or_else(|e| abort!(e.span(), e.to_string()))
                .iter()
                .any(|ident| *ident == "usize")
    });

    if !is_repr_usize {
        abort!(Span::call_site(), "enum must have a `#[repr(usize)]`")
    }

    let syn::Data::Enum(data_enum) = input.data else {
        abort!(
            Span::call_site(),
            "`EnumSelect` can only be derived on enums"
        )
    };

    if data_enum.variants.is_empty() {
        abort!(Span::call_site(), "enum must be non-empty")
    }

    let variants: Vec<proc_macro2::Ident> = data_enum
        .variants
        .into_iter()
        .map(|variant| {
            match variant.fields {
                syn::Fields::Unit => (),
                _ => abort!(variant.fields.span(), "all variants must be a unit variant"),
            };

            if let Some(disc) = variant.discriminant {
                abort!(
                    disc.1.span(),
                    "all variants must have the default discriminant"
                )
            }

            variant.ident
        })
        .collect();

    UnitEnum {
        name: input.ident,
        variants,
    }
}

struct UnitEnum {
    name: syn::Ident,
    variants: Vec<proc_macro2::Ident>,
}
