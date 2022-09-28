#![cfg_attr(not(feature = "std"), no_std)]

use fingerprint_struct::Fingerprint;
use mock_digest::MockDigest;

#[allow(dead_code)] // When all derive tests are disabled
fn assert_same_fingerprint<A: Fingerprint, B: Fingerprint>(a: A, b: B) {
    let mut hasher_a = MockDigest::default();
    a.fingerprint(&mut hasher_a);

    let mut hasher_b = MockDigest::default();
    b.fingerprint(&mut hasher_b);

    assert_eq!(hasher_a.as_ref(), hasher_b.as_ref());
}

#[test]
#[cfg(feature = "derive")]
fn derive_struct_unit() {
    #[derive(Fingerprint)]
    struct Struct;

    assert_same_fingerprint(Struct, ());
}

#[test]
#[cfg(feature = "derive")]
fn derive_struct_tuple() {
    #[derive(Fingerprint)]
    struct Struct(u8, u16, u32);

    assert_same_fingerprint(Struct(1, 2, 3), (1u8, 2u16, 3u32));
}

#[test]
#[cfg(feature = "derive")]
fn derive_struct_fields() {
    #[derive(Fingerprint)]
    struct Struct {
        a: u8,
        b: u16,
        c: u32,
    }

    assert_same_fingerprint(Struct { a: 1, b: 2, c: 3 }, (1u8, 2u16, 3u32));
}

#[test]
#[cfg(feature = "derive")]
fn derive_enum_unit() {
    #[derive(Fingerprint)]
    enum Enum {
        A,
        B,
        C,
    }

    assert_same_fingerprint(Enum::A, 0isize);
    assert_same_fingerprint(Enum::B, 1isize);
    assert_same_fingerprint(Enum::C, 2isize);
}

#[test]
#[cfg(feature = "derive")]
fn derive_enum_unit_discriminant() {
    #[repr(u16)]
    #[derive(Fingerprint)]
    enum Enum {
        A,
        B = 1337,
        C,
        D = 5,
    }

    assert_same_fingerprint(Enum::A, 0u16);
    assert_same_fingerprint(Enum::B, 1337u16);
    assert_same_fingerprint(Enum::C, 1338u16);
    assert_same_fingerprint(Enum::D, 5u16);
}

#[test]
#[cfg(feature = "derive")]
fn derive_enum_tuple() {
    #[derive(Fingerprint)]
    enum Enum {
        A(u8, u16),
        B(u32, i64),
    }

    assert_same_fingerprint(Enum::A(1, 2), (0isize, 1u8, 2u16));
    assert_same_fingerprint(Enum::B(1, 2), (1isize, 1u32, 2u64));
}

#[test]
#[cfg(feature = "derive")]
fn derive_enum_fields() {
    #[derive(Fingerprint)]
    enum Enum {
        A { a: u8, b: u16 },
        B { a: u32, b: i64 },
    }

    assert_same_fingerprint(Enum::A { a: 1, b: 2 }, (0isize, 1u8, 2u16));
    assert_same_fingerprint(Enum::B { a: 1, b: 2 }, (1isize, 1u32, 2u64));
}

#[test]
#[cfg(feature = "derive")]
fn derive_enum_field_named_hasher() {
    #[derive(Fingerprint)]
    enum Enum {
        A { hasher: u32 },
    }

    assert_same_fingerprint(Enum::A { hasher: 1337 }, (0isize, 1337u32));
}

#[test]
#[cfg(feature = "derive")]
fn derive_struct_generic() {
    #[derive(Fingerprint)]
    struct Struct<T>(T);

    assert_same_fingerprint(Struct(1337u32), 1337u32);
}

#[test]
#[cfg(feature = "derive")]
fn derive_struct_generic_bound() {
    #[derive(Fingerprint)]
    struct Struct<T: From<u8>>(T);

    assert_same_fingerprint(Struct(1337u32), 1337u32);
}

#[test]
#[cfg(feature = "derive")]
fn derive_struct_generic_where() {
    #[derive(Fingerprint)]
    struct Struct<T>(T)
    where
        u8: Into<T>;

    assert_same_fingerprint(Struct(1337u32), 1337u32);
}

#[test]
#[cfg(feature = "derive")]
fn derive_struct_generic_as_field_generic() {
    #[derive(Fingerprint)]
    struct Struct<T>(Option<T>);

    assert_same_fingerprint(Struct(Some(1337u32)), Some(1337u32));
}
