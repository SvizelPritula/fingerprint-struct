use std::{
    collections::HashSet,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    num::NonZeroI128,
    sync::Arc,
};

use blake2::Blake2b512;
use fingerprint_struct::fingerprint;
use hex::ToHex;

fn main() {
    let payload = (
        vec![HashSet::from_iter(0..100), HashSet::from_iter(1000..1100)],
        [1, 2, 3],
        NonZeroI128::new(1337),
        Arc::new("🦀"),
        Some(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(127, 0, 0, 1),
            17,
        ))),
    );

    let hash = fingerprint::<Blake2b512>(payload);
    let hash: String = hash.encode_hex_upper();
    println!("{hash}");
}
