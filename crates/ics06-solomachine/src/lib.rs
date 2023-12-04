//! ICS 06: Solomachine light client implementation along with re-exporting data
//! structures and implementations of IBC core client module.
#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![cfg_attr(not(test), deny(clippy::disallowed_methods, clippy::disallowed_types))]
#![deny(
    // warnings,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    rust_2018_idioms
)]

extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

pub mod client_state;
pub mod consensus_state;
pub mod context;
pub mod proof;

pub use context::*;

pub const SOLOMACHINE_CLIENT_TYPE: &str = "06-solomachine";

/// Re-export of Solomachine light client data structures from `ibc-client-solomachine-types` crate.
pub mod types {
    #[doc(inline)]
    pub use ibc_client_solomachine_types::*;
}
