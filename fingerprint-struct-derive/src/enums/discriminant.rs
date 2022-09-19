use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, Expr, Meta, NestedMeta, Path, Variant};

pub struct AddDiscriminant<T: Iterator<Item = Variant>> {
    iterator: T,
    discriminant: Option<Expr>,
    offset: usize,
}

impl<T: Iterator<Item = Variant>> Iterator for AddDiscriminant<T> {
    type Item = (Variant, TokenStream);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(next) => {
                let discriminant: TokenStream = match next.discriminant.clone() {
                    Some((_, discriminant)) => {
                        self.discriminant = Some(discriminant.clone());
                        self.offset = 0;

                        quote!(#discriminant)
                    }
                    None => {
                        let offset = Literal::usize_unsuffixed(self.offset);

                        match &self.discriminant {
                            Some(discriminant) => quote!(#discriminant + #offset),
                            None => quote!(#offset),
                        }
                    }
                };

                self.offset += 1;

                Some((next, discriminant))
            }
            None => None,
        }
    }
}

pub fn add_discriminant<T: Iterator<Item = Variant>>(iterator: T) -> AddDiscriminant<T> {
    AddDiscriminant {
        iterator,
        discriminant: None,
        offset: 0,
    }
}

const PRIMITIVE_NAMES: [&str; 12] = [
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
];

pub fn get_int_repr(attrs: Vec<Attribute>) -> TokenStream {
    let mut repr: Option<Path> = None;

    for meta in attrs.iter().map(extract_repr_arguments).flatten() {
        if let NestedMeta::Meta(Meta::Path(path)) = meta {
            if PRIMITIVE_NAMES.iter().any(|p| check_ident_name(&path, p)) {
                repr = Some(path);
            }
        }
    }

    match repr {
        Some(repr) => repr.to_token_stream(),
        None => quote!(isize),
    }
}

fn extract_repr_arguments(attr: &Attribute) -> Vec<NestedMeta> {
    if !check_ident_name(&attr.path, "repr") {
        return Vec::new();
    }

    match attr.parse_meta() {
        Ok(Meta::List(list)) => list.nested.into_iter().collect(),
        _ => Vec::new(),
    }
}

fn check_ident_name(path: &Path, name: &str) -> bool {
    path.is_ident(&Ident::new(name, Span::mixed_site()))
        || path.is_ident(&Ident::new_raw(name, Span::mixed_site()))
}
