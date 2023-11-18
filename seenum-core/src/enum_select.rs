use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, Token};

pub fn derive(input: TokenStream) -> TokenStream {
    derive_impl(input).unwrap_or_else(syn::Error::into_compile_error)
}

fn derive_impl(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2(input)?;
    let UnitEnum { name, variants } = validate_input(input)?;

    Ok(quote! {
        unsafe impl ::seenum::EnumSelect for #name {
            // SAFETY: `count` is non-zero as validated by `validate_input`.
            const ALL: &'static [Self] = [#(Self::#variants),*].as_slice();

            unsafe fn from_index_unchecked(index: ::core::primitive::usize) -> Self {
                // SAFETY: `index` must be between `0..Self::COUNT`.
                unsafe { ::core::mem::transmute(index) }
            }
        }
    })
}

/// Validates the derive input to be a `#[repr(usize)]` enum with only unit
/// variants, no custom discriminants and at least one variant.
fn validate_input(input: syn::DeriveInput) -> syn::Result<UnitEnum> {
    // using a loop instead of `iter.any` to return errors if necessary
    let mut is_repr_usize = false;
    for attr in input.attrs {
        if attr.path().is_ident("repr")
            && attr
                .parse_args_with(Punctuated::<syn::Ident, Token![,]>::parse_terminated)?
                .iter()
                .any(|ident| *ident == "usize")
        {
            is_repr_usize = true;
            break;
        }
    }

    if !is_repr_usize {
        return Err(syn::Error::new(
            Span::call_site(),
            "enum must have a `#[repr(usize)]`",
        ));
    }

    let syn::Data::Enum(data_enum) = input.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "`EnumSelect` can only be derived on enums",
        ));
    };

    if data_enum.variants.is_empty() {
        return Err(syn::Error::new(Span::call_site(), "enum must be non-empty"));
    }

    let variants: Vec<proc_macro2::Ident> = data_enum
        .variants
        .into_iter()
        .map(|variant| {
            match variant.fields {
                syn::Fields::Unit => (),
                _ => {
                    return Err(syn::Error::new_spanned(
                        variant.fields,
                        "all variants must be a unit variant",
                    ))
                }
            };

            if let Some(disc) = variant.discriminant {
                return Err(syn::Error::new_spanned(
                    disc.1,
                    "all variants must have the default discriminant",
                ));
            }

            Ok(variant.ident)
        })
        .collect::<syn::Result<_>>()?;

    Ok(UnitEnum {
        name: input.ident,
        variants,
    })
}

/// An enum with only unit variants.
struct UnitEnum {
    name: syn::Ident,
    variants: Vec<proc_macro2::Ident>,
}
