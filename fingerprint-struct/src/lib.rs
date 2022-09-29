//! Compute cryptographic hashes of any data
//!
//! This crate provides the [`Fingerprint`] trait, as well as implementations for [`std`] types and
//! a derive macro for generating implementations automatically.
//!
//! Hashes are considered stable, changes to how a given data structure is hashed will cause
//! a minor version bump. Note that a change to your own types might introduce hash collisions.
//! To avoid this, include a version identifier in your data structure, like this:
//!
//! ```
//! use blake2::Blake2b512;
//! use fingerprint_struct::fingerprint;
//!
//! let payload = "Hello world!";
//! let hash = fingerprint::<Blake2b512>((env!("CARGO_PKG_VERSION"), payload));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

use digest::{FixedOutput, Output, Update};

/// A data structure whose cryptographic hash can be computed by a hasher.
///
/// Implementations are provided for common [`std`] types, such as primitives, strings, collections and smart pointers.
/// Custom implementations can be easily written manually, or automatically using `#[derive(Fingerprint)]`.
pub trait Fingerprint {
    /// Use this value to update a hasher.
    fn fingerprint<U: Update>(&self, hasher: &mut U);
}

/// Calculate the cryptographic hash of a data structure using the default hasher of a given type
///
/// # Examples
/// ```
/// use sha2::Sha512;
/// use fingerprint_struct::{fingerprint, Fingerprint};
///
/// let hash = fingerprint::<Sha512>("Hello world!");
/// println!("{hash:?}");
/// ```
///
/// ```
/// use blake2::Blake2b512;
/// use fingerprint_struct::{fingerprint, Fingerprint};
///
/// let hash = fingerprint::<Blake2b512>("Hello world!");
/// println!("{hash:?}");
/// ```
pub fn fingerprint<H: Update + FixedOutput + Default>(value: impl Fingerprint) -> Output<H> {
    fingerprint_with(value, H::default())
}

/// Calculate the cryptographic hash of a data structure using provided hasher
///
/// # Examples
/// ```
/// use blake2::{digest::Digest, Blake2b512};
/// use fingerprint_struct::{fingerprint_with, Fingerprint};
///
/// let hash = fingerprint_with("Hello world!", Blake2b512::new_with_prefix("Application specific prefix"));
/// println!("{hash:?}");
/// ```
pub fn fingerprint_with<H: Update + FixedOutput, T: Fingerprint>(
    value: T,
    mut hasher: H,
) -> Output<H> {
    value.fingerprint(&mut hasher);
    hasher.finalize_fixed()
}

/// Implements the Fingerprint trait for a custom struct or enum.
///
/// Explicit enum discriminants will be used when provided.
///
/// # Examples
/// ```
/// use fingerprint_struct::Fingerprint;
///
/// #[derive(Fingerprint)]
/// struct SearchResult {
///     title: String,
///     link: String,
///     rating: Option<u8>
/// }
/// ```
///
/// ```
/// use fingerprint_struct::Fingerprint;
///
/// #[derive(Fingerprint)]
/// enum LoginState {
///     LoggedOut,
///     LoggedIn { token: String }
/// }
/// ```
#[cfg(feature = "derive")]
pub use fingerprint_struct_derive::Fingerprint;
