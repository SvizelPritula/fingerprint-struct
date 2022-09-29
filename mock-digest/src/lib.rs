//! This crate provides a mock of the [`Update`] trait from the [`digest`] crate.
//! Its primary use is testing the `fingerprint-struct` crate.

use digest::{Reset, Update};

/// A mock of the [`Update`] trait
///
/// Instead of actually hashing its input, it just collects all bytes and stores them.
///
/// # Examples
/// ```
/// use digest::Update;
/// use mock_digest::MockDigest;
///
/// let mut mock_digest = MockDigest::default();
///
/// mock_digest.update(&[1, 2]);
/// mock_digest.update(&[3]);
///
/// assert_eq!(&[1, 2, 3], mock_digest.as_ref());
/// ```
#[derive(Hash, Debug, Clone, Default)]
pub struct MockDigest {
    input: Vec<u8>,
}

impl Update for MockDigest {
    /// Collect and store bytes
    fn update(&mut self, data: &[u8]) {
        self.input.extend_from_slice(data)
    }
}

impl Reset for MockDigest {
    /// Resets hasher to initial state, discarding all stored bytes.
    fn reset(&mut self) {
        self.input = Vec::new();
    }
}

impl From<MockDigest> for Vec<u8> {
    /// Extracts collected bytes.
    fn from(src: MockDigest) -> Self {
        src.input
    }
}

impl AsRef<[u8]> for MockDigest {
    /// Gets collected bytes as a slice.
    fn as_ref(&self) -> &[u8] {
        &self.input
    }
}
