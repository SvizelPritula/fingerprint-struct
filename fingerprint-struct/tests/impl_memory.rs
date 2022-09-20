use std::{borrow::Cow, cell::Cell, rc::Rc, sync::Arc};

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
fn fingerprint_ref() {
    assert_same_fingerprint(&42, 42);
}

#[test]
fn fingerprint_ref_mut() {
    assert_same_fingerprint(&mut 42, 42);
}

#[test]
fn fingerprint_box() {
    assert_same_fingerprint(Box::new(42), 42);
}

#[test]
fn fingerprint_rc() {
    assert_same_fingerprint(Rc::new(42), 42);
}

#[test]
fn fingerprint_arc() {
    assert_same_fingerprint(Arc::new(42), 42);
}

#[test]
fn fingerprint_cow() {
    assert_same_fingerprint(Cow::Borrowed(&42), 42);
}

#[test]
fn fingerprint_cell() {
    assert_same_fingerprint(Cell::new(42), 42);
}
