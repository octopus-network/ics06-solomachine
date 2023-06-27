#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(any(feature = "v2", feature = "v3"))]
pub mod cosmos;
pub mod prelude;

#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v2")]
pub mod v2;
#[cfg(feature = "v3")]
pub mod v3;
