use proc_macro2::{Ident, Span};
use syn::{FieldsNamed, FieldsUnnamed};

pub fn get_field_names(fields: &FieldsNamed) -> impl Iterator<Item = Ident> + '_ {
    fields
        .named
        .iter()
        .map(|field| field.ident.clone().expect("named fields to have names"))
}

pub fn get_unnamed_field_names(fields: &FieldsUnnamed) -> impl Iterator<Item = Ident> + '_ {
    let numbers = 0..fields.unnamed.len();
    numbers
        .into_iter()
        .map(|n| Ident::new(&format!("f{n}"), Span::mixed_site()))
}

pub fn hasher_arg() -> Ident {
    Ident::new(
        "__internal_fingerprint_struct_derive_implementation_hasher_argument",
        Span::mixed_site(),
    )
}
