#![cfg_attr(not(feature = "std"), no_std)]

use core::{
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    sync::atomic::{
        AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
        AtomicU64, AtomicU8, AtomicUsize,
    },
};

use fingerprint_struct::Fingerprint;
use mock_digest::MockDigest;

fn assert_same_fingerprint<A: Fingerprint, B: Fingerprint>(a: A, b: B) {
    let mut hasher_a = MockDigest::default();
    a.fingerprint(&mut hasher_a);

    let mut hasher_b = MockDigest::default();
    b.fingerprint(&mut hasher_b);

    assert_eq!(hasher_a.as_ref(), hasher_b.as_ref());
}

#[test]
fn fingerprint_nonzero_u8() {
    assert_same_fingerprint(NonZeroU8::new(42).unwrap(), 42u8);
}
#[test]
fn fingerprint_nonzero_u16() {
    assert_same_fingerprint(NonZeroU16::new(42).unwrap(), 42u16);
}
#[test]
fn fingerprint_nonzero_u32() {
    assert_same_fingerprint(NonZeroU32::new(42).unwrap(), 42u32);
}
#[test]
fn fingerprint_nonzero_u64() {
    assert_same_fingerprint(NonZeroU64::new(42).unwrap(), 42u64);
}
#[test]
fn fingerprint_nonzero_u128() {
    assert_same_fingerprint(NonZeroU128::new(42).unwrap(), 42u128);
}
#[test]
fn fingerprint_nonzero_i8() {
    assert_same_fingerprint(NonZeroI8::new(42).unwrap(), 42i8);
}
#[test]
fn fingerprint_nonzero_i16() {
    assert_same_fingerprint(NonZeroI16::new(42).unwrap(), 42i16);
}
#[test]
fn fingerprint_nonzero_i32() {
    assert_same_fingerprint(NonZeroI32::new(42).unwrap(), 42i32);
}
#[test]
fn fingerprint_nonzero_i64() {
    assert_same_fingerprint(NonZeroI64::new(42).unwrap(), 42i64);
}
#[test]
fn fingerprint_nonzero_i128() {
    assert_same_fingerprint(NonZeroI128::new(42).unwrap(), 42i128);
}
#[test]
fn fingerprint_nonzero_usize() {
    assert_same_fingerprint(NonZeroUsize::new(42).unwrap(), 42usize);
}
#[test]
fn fingerprint_nonzero_isize() {
    assert_same_fingerprint(NonZeroIsize::new(42).unwrap(), 42isize);
}

#[test]
fn fingerprint_atomic_u8() {
    assert_same_fingerprint(AtomicU8::new(42), 42u8);
}
#[test]
fn fingerprint_atomic_u16() {
    assert_same_fingerprint(AtomicU16::new(42), 42u16);
}
#[test]
fn fingerprint_atomic_u32() {
    assert_same_fingerprint(AtomicU32::new(42), 42u32);
}
#[test]
fn fingerprint_atomic_u64() {
    assert_same_fingerprint(AtomicU64::new(42), 42u64);
}
#[test]
fn fingerprint_atomic_i8() {
    assert_same_fingerprint(AtomicI8::new(42), 42i8);
}
#[test]
fn fingerprint_atomic_i16() {
    assert_same_fingerprint(AtomicI16::new(42), 42i16);
}
#[test]
fn fingerprint_atomic_i32() {
    assert_same_fingerprint(AtomicI32::new(42), 42i32);
}
#[test]
fn fingerprint_atomic_i64() {
    assert_same_fingerprint(AtomicI64::new(42), 42i64);
}
#[test]
fn fingerprint_atomic_usize() {
    assert_same_fingerprint(AtomicUsize::new(42), 42usize);
}
#[test]
fn fingerprint_atomic_isize() {
    assert_same_fingerprint(AtomicIsize::new(42), 42isize);
}
#[test]
fn fingerprint_atomic_bool() {
    assert_same_fingerprint(AtomicBool::new(false), false);
    assert_same_fingerprint(AtomicBool::new(true), true);
}
