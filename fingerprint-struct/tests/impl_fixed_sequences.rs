#![cfg_attr(not(feature = "std"), no_std)]

use fingerprint_struct::Fingerprint;
use mock_digest::MockDigest;

fn assert_fingerprint<T: Fingerprint>(value: T, fingerprint: &[u8]) {
    let mut hasher = MockDigest::default();
    value.fingerprint(&mut hasher);
    assert_eq!(hasher.as_ref(), fingerprint);
}

#[test]
fn fingerprint_u8_array() {
    assert_fingerprint([1u8, 2u8, 3u8, 4u8, 5u8, 6u8], &[1, 2, 3, 4, 5, 6]);
}

#[test]
fn fingerprint_u32_array() {
    assert_fingerprint(&[10, 20], &[10, 0, 0, 0, 20, 0, 0, 0]);
}

#[test]
fn fingerprint_unit() {
    assert_fingerprint((), &[]);
}

#[test]
fn fingerprint_tuple_1() {
    assert_fingerprint((0x1337u16,), &[0x37, 0x13]);
}

#[test]
fn fingerprint_tuple_2() {
    assert_fingerprint((0xbeefu16, 0xffu8), &[0xef, 0xbe, 0xff]);
}

#[test]
fn fingerprint_tuple_16() {
    assert_fingerprint(
        (
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
        ),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    );
}

#[test]
fn fingerprint_tuple_nested() {
    assert_fingerprint(
        ((0u8, (10u8, 11u8)), (20u8, (21u8,), 22u8)),
        &[0, 10, 11, 20, 21, 22],
    );
}

#[test]
fn fingerprint_str() {
    assert_fingerprint(
        "příklad",
        &[9, 0x70, 0xc5, 0x99, 0xc3, 0xad, 0x6b, 0x6c, 0x61, 0x64],
    );
}
