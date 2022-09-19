use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use enums::get_enum_fn_body;
use structs::get_struct_body;

mod enums;
mod structs;
mod utils;

#[proc_macro_derive(Fingerprint)]
pub fn derive_fingerprint(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let DeriveInput { ident, data, attrs, .. } = input;

    let body = match data {
        syn::Data::Struct(data) => get_struct_body(data),
        syn::Data::Enum(data) => get_enum_fn_body(data, &ident, attrs),
        syn::Data::Union(_) => quote!(compile_error!("cannot derive Fingerprint for an union")),
    };

    quote! {
        impl ::fingerprint_struct::Fingerprint for #ident {
            fn fingerprint<U: ::digest::Update>(&self, hasher: &mut U) {
                #body
            }
        }
    }
    .into()
}
