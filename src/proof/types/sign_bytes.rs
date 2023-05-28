use crate::error::Error;
use crate::prelude::*;
use core::str::FromStr;
use ibc::core::ics24_host::path::Path;
use ibc_proto::ibc::lightclients::solomachine::v3::SignBytes as RawSignBytes;

use ibc_proto::protobuf::Protobuf;

/// SignBytes defines the signed bytes used for signature verification.
#[derive(Clone, PartialEq)]
pub struct SignBytes {
    /// the sequence number
    pub sequence: u64,
    /// the proof timestamp
    pub timestamp: u64,
    /// the public key diversifier
    pub diversifier: String,
    /// the standardised path bytes
    pub path: Vec<u8>,
    /// the marshaled data bytes
    pub data: Vec<u8>,
}

impl Protobuf<RawSignBytes> for SignBytes {}

impl TryFrom<RawSignBytes> for SignBytes {
    type Error = Error;

    fn try_from(raw: RawSignBytes) -> Result<Self, Self::Error> {
        Ok(Self {
            sequence: raw.sequence,
            timestamp: raw.timestamp,
            diversifier: raw.diversifier,
            path: raw.path,
            data: raw.data,
        })
    }
}

impl From<SignBytes> for RawSignBytes {
    fn from(value: SignBytes) -> Self {
        Self {
            sequence: value.sequence,
            timestamp: value.timestamp,
            diversifier: value.diversifier,
            path: value.path,
            data: value.data,
        }
    }
}
