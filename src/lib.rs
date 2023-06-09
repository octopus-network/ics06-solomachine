#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod cosmos;
pub mod prelude;

#[cfg(feature = "v2")]
pub mod v2;
#[cfg(feature = "v3")]
pub mod v3;
