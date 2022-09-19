#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;

use digest::Update;

pub trait Fingerprint {
    fn fingerprint<U: Update>(&self, hasher: &mut U);
}

pub use fingerprint_struct_derive::Fingerprint;
