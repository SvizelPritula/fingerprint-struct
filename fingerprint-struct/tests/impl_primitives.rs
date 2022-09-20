#![cfg_attr(not(feature = "std"), no_std)]

use fingerprint_struct::Fingerprint;
use mock_digest::MockDigest;

fn assert_fingerprint<T: Fingerprint>(value: T, fingerprint: &[u8]) {
    let mut hasher = MockDigest::default();
    value.fingerprint(&mut hasher);
    assert_eq!(hasher.as_ref(), fingerprint);
}

#[test]
fn fingerprint_u8() {
    assert_fingerprint(42u8, &[42]);
}

#[test]
fn fingerprint_u16() {
    assert_fingerprint(0x1337u16, &[0x37, 0x13]);
}

#[test]
fn fingerprint_u32() {
    assert_fingerprint(0x1337beefu32, &[0xef, 0xbe, 0x37, 0x13]);
}

#[test]
fn fingerprint_u64() {
    assert_fingerprint(
        0x7766554433221100u64,
        &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77],
    );
}

#[test]
fn fingerprint_u128() {
    assert_fingerprint(
        0xffeeddccbbaa99887766554433221100u128,
        &[
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ],
    );
}

#[test]
fn fingerprint_i8() {
    assert_fingerprint(-2i8, &[0xfe]);
}

#[test]
fn fingerprint_i16() {
    assert_fingerprint(-2i16, &[0xfe, 0xff]);
}

#[test]
fn fingerprint_i32() {
    assert_fingerprint(-2i32, &[0xfe, 0xff, 0xff, 0xff]);
}

#[test]
fn fingerprint_i64() {
    assert_fingerprint(-2i64, &[0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
}

#[test]
fn fingerprint_i128() {
    assert_fingerprint(
        -2i128,
        &[
            0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
    );
}

#[test]
fn fingerprint_f32() {
    assert_fingerprint(12.34f32, &[0xa4, 0x70, 0x45, 0x41]);
    assert_fingerprint(-12.34f32, &[0xa4, 0x70, 0x45, 0xc1]);
    assert_fingerprint(f32::NAN, &[0x00, 0x00, 0xc0, 0x7f]);
    assert_fingerprint(f32::INFINITY, &[0x00, 0x00, 0x80, 0x7f]);
}

#[test]
fn fingerprint_f64() {
    assert_fingerprint(12.34f64, &[0xae, 0x47, 0xe1, 0x7a, 0x14, 0xae, 0x28, 0x40]);
    assert_fingerprint(-12.34f64, &[0xae, 0x47, 0xe1, 0x7a, 0x14, 0xae, 0x28, 0xc0]);
    assert_fingerprint(f64::NAN, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x7f]);
    assert_fingerprint(
        f64::INFINITY,
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x7f],
    );
}

#[test]
fn fingerprint_usize() {
    assert_fingerprint(0usize, &[0]);
    assert_fingerprint(0b11111110000000usize, &[0b10000000, 0b01111111]);
}

#[test]
fn fingerprint_isize() {
    assert_fingerprint(0isize, &[0]);
    assert_fingerprint(-1isize, &[1]);
    assert_fingerprint(1isize, &[2]);
    assert_fingerprint(-2isize, &[3]);
    assert_fingerprint(2isize, &[4]);

    assert_fingerprint(-0b1111111isize, &[0b11111101, 0b00000001]);
}

#[test]
fn fingerprint_char() {
    assert_fingerprint('a', &[0x61, 0, 0, 0]);
    assert_fingerprint('🦀', &[0x80, 0xf9, 0x01, 0]);
}

#[test]
fn fingerprint_bool() {
    assert_fingerprint(false, &[0]);
    assert_fingerprint(true, &[1]);
}
