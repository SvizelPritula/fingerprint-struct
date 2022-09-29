use blake2::Blake2b512;
use fingerprint_struct::fingerprint;
use hex::ToHex;

fn main() {
    let payload = "Hello world!";
    let hash = fingerprint::<Blake2b512>((env!("CARGO_PKG_VERSION"), payload));
    let hash: String = hash.encode_hex_upper();
    println!("{hash}");
}
