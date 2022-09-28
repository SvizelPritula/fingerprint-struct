use blake2::{Blake2b512, Digest};
use fingerprint_struct::fingerprint;
use hex::ToHex;

fn main() {
    let payload = "Hello world!";
    let hash = fingerprint(payload, Blake2b512::new());
    let hash: String = hash.encode_hex_upper();
    println!("{}", hash);
}
