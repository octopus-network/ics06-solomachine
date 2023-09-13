#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::str::FromStr;
use ibc::core::ics02_client::client_type::ClientType;

// for crypto of cosmos
pub mod cosmos;
pub mod prelude;

pub mod client_state;
pub mod consensus_state;
pub mod context;
pub mod error;
pub mod header;
pub mod misbehaviour;
pub mod proof;

pub use context::*;

pub(crate) const SOLOMACHINE_CLIENT_TYPE: &str = "06-solomachine";

pub fn client_type() -> ClientType {
    ClientType::from_str(SOLOMACHINE_CLIENT_TYPE).expect("invalid client type")
}
