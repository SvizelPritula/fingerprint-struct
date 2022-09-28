use blake2::{Blake2b512, Digest};
use fingerprint_struct::Fingerprint;
use hex::ToHex;

fn main() {
    let mut digest = Blake2b512::new();

    1.fingerprint(&mut digest);
    2.fingerprint(&mut digest);
    3.fingerprint(&mut digest);

    let hash = digest.finalize();
    let hash: String = hash.encode_hex_upper();
    println!("{}", hash);
}
