use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, ConstParam, Data, Fields, Generics, LifetimeDef, Token, TypeParam,
};

pub enum GenericParamType {
    Impl,
    Type,
}

pub fn get_generic_parameters(generics: &Generics, target: GenericParamType) -> TokenStream {
    let params = generics.params.iter().map(|param| match param {
        syn::GenericParam::Type(param) => param.ident.to_token_stream(),
        syn::GenericParam::Lifetime(param) => param.lifetime.to_token_stream(),
        syn::GenericParam::Const(ConstParam { ident, ty, .. }) => match target {
            GenericParamType::Impl => quote!(const #ident: #ty),
            GenericParamType::Type => ident.to_token_stream(),
        },
    });
    let params: Punctuated<_, Token!(,)> = params.collect();

    params.to_token_stream()
}

pub fn get_where_bounds(generics: &Generics, data: &Data) -> TokenStream {
    let mut bounds: Punctuated<TokenStream, Token!(,)> = Punctuated::new();

    if let Some(clause) = &generics.where_clause {
        bounds.extend(clause.predicates.iter().map(|c| c.to_token_stream()));
    }

    bounds.extend(get_where_bounds_from_params(generics));

    match data {
        Data::Struct(data) => bounds.extend(get_where_bounds_from_fields(&data.fields)),
        Data::Enum(data) => {
            for variant in data.variants.iter() {
                bounds.extend(get_where_bounds_from_fields(&variant.fields));
            }
        }
        Data::Union(_) => {}
    };

    bounds.to_token_stream()
}

fn get_where_bounds_from_params(generics: &Generics) -> Punctuated<TokenStream, Token!(,)> {
    let bounds: Punctuated<TokenStream, Token!(,)> = generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Type(TypeParam { ident, bounds, .. }) => {
                if bounds.len() > 0 {
                    Some(quote!(#ident: #bounds))
                } else {
                    None
                }
            }
            syn::GenericParam::Lifetime(LifetimeDef {
                lifetime, bounds, ..
            }) => {
                if bounds.len() > 0 {
                    Some(quote!(#lifetime: #bounds))
                } else {
                    None
                }
            }
            syn::GenericParam::Const(_) => None,
        })
        .filter_map(|b| b)
        .collect();

    bounds
}

fn get_where_bounds_from_fields(fields: &Fields) -> Punctuated<TokenStream, Token!(,)> {
    let types = match fields {
        Fields::Named(fields) => Some(fields.named.iter()),
        Fields::Unnamed(fields) => Some(fields.unnamed.iter()),
        Fields::Unit => None,
    };

    match types {
        Some(types) => {
            let bounds: Punctuated<TokenStream, Token!(,)> = types
                .map(|field| field.ty.clone())
                .map(|ty| quote!(#ty: ::fingerprint_struct::Fingerprint))
                .collect();

            bounds
        }
        None => Punctuated::new(),
    }
}
