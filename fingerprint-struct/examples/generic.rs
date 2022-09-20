use digest::Update;
use fingerprint_struct::Fingerprint;

#[derive(Fingerprint)]
struct Simple<T>(T);

#[derive(Fingerprint)]
struct ParameterBound<T: Into<u32>>(T);

#[derive(Fingerprint)]
struct WhereClause<T>(T)
where
    T: Into<u32>;

#[derive(Fingerprint)]
struct Const<const N: usize>([u8; N]);

#[derive(Fingerprint)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {
    let mut hasher = MockDigest::default();

    Simple(1u8).fingerprint(&mut hasher);
    ParameterBound(1u8).fingerprint(&mut hasher);
    WhereClause(2u8).fingerprint(&mut hasher);
    Const([3u8, 4u8]).fingerprint(&mut hasher);
    Enum::<u8, u8>::A(5u8).fingerprint(&mut hasher);
    Enum::<u8, u8>::B(6u8).fingerprint(&mut hasher);

    println!("{:?}", hasher.as_bytes());
}

#[derive(Default)]
struct MockDigest {
    bytes: Vec<u8>,
}

impl MockDigest {
    fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

impl Update for MockDigest {
    fn update(&mut self, data: &[u8]) {
        self.bytes.extend_from_slice(data)
    }
}
