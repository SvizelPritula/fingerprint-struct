#![cfg_attr(not(feature = "std"), no_std)]

use blake2::{Blake2s256, Digest};
use fingerprint_struct::fingerprint;

#[test]
fn fingerprint_func() {
    let hash = fingerprint(("Hello world", 1337), Blake2s256::new());

    assert_eq!(
        <[u8; 32]>::from(hash),
        [
            7, 111, 119, 103, 16, 73, 77, 122, 160, 198, 220, 50, 209, 55, 161, 211, 88, 74, 219,
            113, 49, 245, 73, 75, 91, 147, 101, 55, 98, 143, 206, 36
        ]
    );
}
