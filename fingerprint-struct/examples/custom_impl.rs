use blake2::Blake2b512;
use fingerprint_struct::{fingerprint, Fingerprint};
use hex::ToHex;

struct Color(u8, u8, u8);

impl Fingerprint for Color {
    fn fingerprint<U: digest::Update>(&self, hasher: &mut U) {
        self.0.fingerprint(hasher);
        self.1.fingerprint(hasher);
        self.2.fingerprint(hasher);
    }
}

fn main() {
    let payload = Color(0xdc, 0x14, 0x3c);
    let hash = fingerprint::<Blake2b512>(payload);
    let hash: String = hash.encode_hex_upper();
    println!("{}", hash);
}
