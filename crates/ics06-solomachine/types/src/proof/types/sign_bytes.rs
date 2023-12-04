use crate::error::Error;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use ibc_proto::ibc::core::commitment::v1::MerklePath;
use ibc_proto::ibc::lightclients::solomachine::v3::SignBytes as RawSignBytes;
use ibc_proto::Protobuf;
use prost::Message;

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
    pub path: MerklePath,
    /// the marshaled data bytes
    pub data: Vec<u8>,
}

impl Protobuf<RawSignBytes> for SignBytes {}

impl TryFrom<RawSignBytes> for SignBytes {
    type Error = Error;

    fn try_from(raw: RawSignBytes) -> Result<Self, Self::Error> {
        let path = MerklePath::decode(raw.path.as_ref())
            .map_err(|e| Error::Other(format!("decode MerklePath Failed({})", e)))?;
        Ok(Self {
            sequence: raw.sequence,
            timestamp: raw.timestamp,
            diversifier: raw.diversifier,
            path,
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
            path: value.path.encode_to_vec(),
            data: value.data,
        }
    }
}
