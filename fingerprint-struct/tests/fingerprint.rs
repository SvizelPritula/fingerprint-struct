#![cfg_attr(not(feature = "std"), no_std)]

use blake2::{digest::Digest, Blake2s256};
use fingerprint_struct::{fingerprint, fingerprint_with};

#[test]
fn fingerprint_func() {
    let hash = fingerprint::<Blake2s256>(("Hello world", 1337));

    assert_eq!(
        <[u8; 32]>::from(hash),
        [
            7, 111, 119, 103, 16, 73, 77, 122, 160, 198, 220, 50, 209, 55, 161, 211, 88, 74, 219,
            113, 49, 245, 73, 75, 91, 147, 101, 55, 98, 143, 206, 36
        ]
    );
}

#[test]
fn fingerprint_with_func() {
    let hash = fingerprint_with(("Hello world", 1337), Blake2s256::new());

    assert_eq!(
        <[u8; 32]>::from(hash),
        [
            7, 111, 119, 103, 16, 73, 77, 122, 160, 198, 220, 50, 209, 55, 161, 211, 88, 74, 219,
            113, 49, 245, 73, 75, 91, 147, 101, 55, 98, 143, 206, 36
        ]
    );
}
