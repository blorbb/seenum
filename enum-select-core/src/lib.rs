use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, Token};

pub fn derive_display(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let syn::Data::Enum(data_enum) = input.data else {
        abort!(Span::call_site(), "`Display` is only supported on enums")
    };

    let pairs: Vec<(proc_macro2::Ident, TokenStream)> = data_enum
        .variants
        .iter()
        .map(|variant| {
            let display_attr = variant
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("display"))
                .unwrap_or_else(|| {
                    abort!(
                        variant.span(),
                        "`#[display(...)]` attribute required for all variants"
                    )
                });
            let inner: TokenStream = display_attr.parse_args()?;

            Ok((variant.ident.clone(), inner))
        })
        .collect::<syn::Result<Vec<_>>>()?;

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

pub fn derive_enum_select(input: TokenStream) -> TokenStream {
    let input = syn::parse2(input).unwrap_or_else(|e| abort!(e.span(), e.to_string()));
    let UnitEnum { name, variants } = validate_input(input);

    let count = variants.len();
    let slice_inner: TokenStream = variants
        .iter()
        .map(|ident| quote! { Self::#ident, })
        .collect();

    quote! {
        unsafe impl ::enum_select::EnumSelect for #name {
            const COUNT: ::std::num::NonZeroUsize =
                unsafe { ::std::num::NonZeroUsize::new_unchecked(#count) };

            unsafe fn from_index_unchecked(index: ::std::primitive::usize) -> Self {
                unsafe { ::std::mem::transmute(index) }
            }

            fn as_slice() -> &'static [Self] {
                [#slice_inner].as_slice()
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
