//! ICS-06: Solomachine Client implements a client verification algorithm for blockchains which use
//! the Solomachine consensus algorithm.
#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![cfg_attr(not(test), deny(clippy::disallowed_methods, clippy::disallowed_types,))]
#![deny(
    warnings,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    rust_2018_idioms
)]

use core::str::FromStr;

use ibc_core::host::types::identifiers::ClientType;

extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

pub mod client_state;
pub mod consensus_state;
pub mod cosmos;
pub mod error;
pub mod header;
pub mod misbehaviour;
pub mod proof;

pub use client_state::*;
pub use consensus_state::*;
pub use header::*;
pub use misbehaviour::*;

/// Re-exports necessary proto types for Solomachine light client implementation
/// from `ibc-proto` crate.
pub mod proto {
    pub use ibc_proto::google::protobuf::Any;
    pub use ibc_proto::ibc::lightclients::tendermint::*;
    pub use ibc_proto::Protobuf;
}

pub const SOLOMACHINE_CLIENT_TYPE: &str = "06-solomachine";

/// Returns the tendermint `ClientType`
pub fn client_type() -> ClientType {
    ClientType::from_str(SOLOMACHINE_CLIENT_TYPE).expect("Never fails because it's valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ensures that the validation in `ClientType::from_str` doesn't fail for the solomachine client type
    #[test]
    pub fn test_sm_client_type() {
        let _ = ClientType::from_str(SOLOMACHINE_CLIENT_TYPE).unwrap();
    }
}
