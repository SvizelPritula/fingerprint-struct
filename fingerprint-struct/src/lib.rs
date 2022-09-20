#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

use digest::{FixedOutput, Output, Update};

pub trait Fingerprint {
    fn fingerprint<U: Update>(&self, hasher: &mut U);
}

pub fn fingerprint<H: Update + FixedOutput, T: Fingerprint>(value: T, mut hasher: H) -> Output<H> {
    value.fingerprint(&mut hasher);
    hasher.finalize_fixed()
}

#[cfg(feature = "derive")]
pub use fingerprint_struct_derive::Fingerprint;
