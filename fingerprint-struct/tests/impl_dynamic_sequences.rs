#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
    string::ToString,
    vec,
    vec::Vec,
};
#[cfg(feature = "std")]
use std::{
    collections::{HashMap, HashSet},
    ffi::CString,
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
fn fingerprint_array_slice() {
    let data: &[u8] = &[1, 2, 3, 4];

    assert_same_fingerprint(data, &[4u8, 1, 2, 3, 4]);
}

#[test]
#[cfg(feature = "alloc")]
fn fingerprint_string() {
    assert_same_fingerprint("příklad".to_string(), "příklad");
}

#[test]
#[cfg(feature = "std")]
fn fingerprint_cstr() {
    {
        let str = CString::new("příklad").unwrap();

        assert_same_fingerprint(
            str.as_c_str(),
            &[0x70, 0xc5, 0x99, 0xc3, 0xad, 0x6b, 0x6c, 0x61, 0x64, 0u8],
        );
    }
}

#[test]
#[cfg(feature = "std")]
fn fingerprint_cstring() {
    let str = CString::new("příklad").unwrap();

    assert_same_fingerprint(str.clone(), str.as_c_str());
}

#[test]
#[cfg(feature = "alloc")]
fn fingerprint_vec() {
    assert_same_fingerprint(vec![1u8, 2u8, 3u8, 4u8], &[4u8, 1, 2, 3, 4]);
}

#[test]
#[cfg(feature = "alloc")]
fn fingerprint_linked_list() {
    assert_same_fingerprint(LinkedList::from([1, 2, 3, 4]), vec![1, 2, 3, 4]);
}

#[test]
#[cfg(feature = "alloc")]
fn fingerprint_vec_deque() {
    assert_same_fingerprint(VecDeque::from([1, 2, 3, 4]), vec![1, 2, 3, 4]);
}

#[test]
#[cfg(feature = "alloc")]
fn fingerprint_btree_set() {
    assert_same_fingerprint(BTreeSet::from([4, 2, 3, 1]), vec![1, 2, 3, 4]);
}

#[test]
#[cfg(feature = "alloc")]
fn fingerprint_binary_heap() {
    assert_same_fingerprint(BinaryHeap::from([4, 2, 3, 1]), vec![1, 2, 3, 4]);
}

#[test]
#[cfg(feature = "std")]
fn fingerprint_hash_set() {
    assert_same_fingerprint(HashSet::from([4, 2, 3, 1]), vec![1, 2, 3, 4]);
}

#[test]
#[cfg(feature = "alloc")]
fn fingerprint_btree_map() {
    assert_same_fingerprint(
        BTreeMap::<u32, i16>::from([(4, 1), (2, 2), (3, 3), (1, 4)]),
        Vec::<(u32, i16)>::from([(1, 4), (2, 2), (3, 3), (4, 1)]),
    );
}

#[test]
#[cfg(feature = "std")]
fn fingerprint_hash_map() {
    assert_same_fingerprint(
        HashMap::<u32, i16>::from([(4, 1), (2, 2), (3, 3), (1, 4)]),
        Vec::<(u32, i16)>::from([(1, 4), (2, 2), (3, 3), (4, 1)]),
    );
}
