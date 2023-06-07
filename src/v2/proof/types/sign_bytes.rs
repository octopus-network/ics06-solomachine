use crate::prelude::*;
use crate::v2::error::Error;
use crate::v2::proof::types::DataType;
use ibc_proto::ibc::lightclients::solomachine::v2::SignBytes as RawSignBytes;
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
    /// type of the data used
    pub data_type: DataType,
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
            data_type: DataType::try_from(raw.data_type)?,
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
            data_type: i32::from(value.data_type),
            data: value.data,
        }
    }
}
