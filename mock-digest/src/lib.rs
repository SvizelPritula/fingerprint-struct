use digest::{Reset, Update};

#[derive(Hash, Debug, Clone, Default)]
pub struct MockDigest {
    input: Vec<u8>,
}

impl Update for MockDigest {
    fn update(&mut self, data: &[u8]) {
        self.input.extend_from_slice(data)
    }
}

impl Reset for MockDigest {
    fn reset(&mut self) {
        self.input = Vec::new();
    }
}

impl From<MockDigest> for Vec<u8> {
    fn from(src: MockDigest) -> Self {
        src.input
    }
}

impl AsRef<[u8]> for MockDigest {
    fn as_ref(&self) -> &[u8] {
        &self.input
    }
}
