//! Compute cryptographic hashes of any data
//!
//! This crate provides the [`Fingerprint`] trait, as well as implementations for [`std`] types and
//! a derive macro for generating implementations automatically.
//!
//! Hashes are considered stable, changes to how a given data structure is hashed will cause a minor
//! version bump.

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

use digest::{FixedOutput, Output, Update};

/// A data structure whose cryptographic hash can be computed by a hasher.
///
/// Implementations are provided for common [`std`] types, such as primitives, strings, collections and smart pointers.
/// Custom implementations can be easly written manually, or automaticaly using `#[derive(Fingerprint)]`.
pub trait Fingerprint {
    /// Use this value to update a hasher.
    fn fingerprint<U: Update>(&self, hasher: &mut U);
}

pub fn fingerprint<H: Update + FixedOutput + Default, T: Fingerprint>(value: T) -> Output<H> {
    fingerprint_with(value, H::default())
}

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
