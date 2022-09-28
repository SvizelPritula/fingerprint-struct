use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Attribute, DataEnum, Fields, Variant};

use crate::utils::{get_field_names, get_unnamed_field_names, hasher_arg};

use self::discriminant::{add_discriminant, get_int_repr};

mod discriminant;

pub fn get_enum_fn_body(data: DataEnum, name: &Ident, attrs: Vec<Attribute>) -> TokenStream {
    let DataEnum { variants, .. } = data;

    if variants.len() == 0 {
        return TokenStream::default();
    }

    let int_repr = get_int_repr(attrs);

    let variants = variants.into_iter();
    let variants = add_discriminant(variants);

    let arms = variants.map(|(v, d)| get_match_arm(v, d, name.clone(), &int_repr));

    let arms: TokenStream = arms.collect();

    quote! {
        match self {
            #arms
        }
    }
}

fn get_match_arm(
    variant: Variant,
    discriminant: TokenStream,
    enum_name: Ident,
    int_repr: &TokenStream,
) -> TokenStream {
    let Variant { ident, fields, .. } = variant;

    let body = get_match_body(&fields);
    let pattern = get_match_pattern(&fields);

    let hasher_arg = hasher_arg();

    quote!(
        #enum_name::#ident #pattern => {
            {
                let discriminant: ::core::primitive::#int_repr = #discriminant;
                discriminant.fingerprint(#hasher_arg);
            }
            #body
        }
    )
}

fn get_match_pattern(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(fields) => {
            let idents = get_field_names(fields);
            let body: Punctuated<_, Comma> = Punctuated::from_iter(idents);

            quote!({#body})
        }
        Fields::Unnamed(fields) => {
            let idents = get_unnamed_field_names(fields);
            let body: Punctuated<_, Comma> = Punctuated::from_iter(idents);

            quote!((#body))
        }
        Fields::Unit => TokenStream::default(),
    }
}

fn get_match_body(fields: &Fields) -> TokenStream {
    let hasher_arg = hasher_arg();

    match fields {
        Fields::Named(fields) => {
            let idents = get_field_names(fields);

            let statements = idents.map(|ident| {
                quote! {
                    #ident.fingerprint(#hasher_arg);
                }
            });

            statements.collect()
        }
        Fields::Unnamed(fields) => {
            let idents = get_unnamed_field_names(fields);

            let statements = idents.map(|ident| {
                quote! {
                    #ident.fingerprint(#hasher_arg);
                }
            });

            statements.collect()
        }
        Fields::Unit => TokenStream::default(),
    }
}
