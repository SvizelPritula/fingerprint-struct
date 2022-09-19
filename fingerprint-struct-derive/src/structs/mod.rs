use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields, Index};

use crate::utils::get_field_names;

pub fn get_struct_body(data: DataStruct) -> TokenStream {
    let DataStruct { fields, .. } = data;

    match fields {
        Fields::Named(fields) => {
            let idents = get_field_names(&fields);

            let statements = idents.map(|ident| {
                quote! {
                    self.#ident.fingerprint(hasher);
                }
            });

            statements.collect()
        }
        Fields::Unnamed(fields) => {
            let numbers = 0..fields.unnamed.len();
            let numbers = numbers.into_iter().map(Index::from);

            let statements = numbers.map(|num| {
                quote! {
                    self.#num.fingerprint(hasher);
                }
            });

            statements.collect()
        }
        Fields::Unit => TokenStream::default(),
    }
}
