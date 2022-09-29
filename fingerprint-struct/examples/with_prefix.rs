use blake2::{digest::Digest, Blake2b512};
use fingerprint_struct::fingerprint_with;
use hex::ToHex;

fn main() {
    let payload = "Hello world!";
    let hash = fingerprint_with(
        payload,
        Blake2b512::new_with_prefix("Blake2 prefix example"),
    );
    let hash: String = hash.encode_hex_upper();
    println!("{hash}");
}
