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

impl Into<Vec<u8>> for MockDigest {
    fn into(self) -> Vec<u8> {
        self.input
    }
}

impl AsRef<[u8]> for MockDigest {
    fn as_ref(&self) -> &[u8] {
        &self.input
    }
}
