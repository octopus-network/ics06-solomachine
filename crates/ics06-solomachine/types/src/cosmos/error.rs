//! Error types

use alloc::string::String;
use displaydoc::Display;
use tendermint::Hash;

/// Kinds of errors.
#[derive(Clone, Debug, Eq, Display, PartialEq)]
pub enum Error {
    /// invalid account ID: {id:?}"
    AccountId {
        /// Malformed account ID
        id: String,
    },

    /// Cryptographic errors.
    Crypto,

    /// Invalid decimal value: {value:?}
    Decimal {
        /// Invalid decimal value
        value: String,
    },

    /// Invalid denomination: {name:?}
    Denom {
        /// Invalid name
        name: String,
    },

    /// Invalid value for the given field of an enum, invalid proto enum value: {name:?}, value: {found_value:?}
    InvalidEnumValue {
        /// Name of the enum field
        name: &'static str,

        /// Actual value of the field found
        found_value: i32,
    },

    /// Protobuf is missing a field, missing proto field: {name:?}
    MissingField {
        /// Name of the missing field
        name: &'static str,
    },

    /// Unexpected message type, unexpected Msg type: {found:?}, expected {expected:?}
    MsgType {
        /// Expected type URL.
        expected: &'static str,

        /// Actual type URL found in the [`crate::Any`] message.
        found: String,
    },

    /// Transaction not found: {hash:?}
    TxNotFound {
        /// Transaction hash that wasn't found.
        hash: Hash,
    },

    /// other error: {description},
    Other { description: String },
}
