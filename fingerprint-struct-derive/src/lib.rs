//! This crate provides the implementation of the [`Fingerprint`] derive macro.
//!
//! It's reexported by the `fingerprint-struct` crate when the `derive` feature flag is enabled.

use generics::{get_generic_parameters, get_where_bounds, GenericParamType};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use enums::get_enum_fn_body;
use structs::get_struct_body;
use utils::hasher_arg;

mod enums;
mod generics;
mod structs;
mod utils;

#[proc_macro_derive(Fingerprint)]
pub fn derive_fingerprint(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let DeriveInput {
        ident,
        data,
        attrs,
        generics,
        ..
    } = input;

    let where_bounds = get_where_bounds(&generics, &data);
    let generic_params_impl = get_generic_parameters(&generics, GenericParamType::Impl);
    let generic_params_type = get_generic_parameters(&generics, GenericParamType::Type);

    let body = match data {
        syn::Data::Struct(data) => get_struct_body(data),
        syn::Data::Enum(data) => get_enum_fn_body(data, &ident, attrs),
        syn::Data::Union(_) => quote!(compile_error!("cannot derive Fingerprint for an union")),
    };

    let hasher_arg = hasher_arg();

    quote! {
        impl <#generic_params_impl> ::fingerprint_struct::Fingerprint for #ident <#generic_params_type> where #where_bounds {
            fn fingerprint<U: ::digest::Update>(&self, #hasher_arg: &mut U) {
                #body
            }
        }
    }
    .into()
}
