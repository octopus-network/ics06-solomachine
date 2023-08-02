use crate::prelude::*;
use crate::v3::error::Error;
use ibc::core::timestamp::Timestamp;
use ibc_proto::ibc::lightclients::solomachine::v3::SignatureAndData as RawSignatureAndData;
use ibc_proto::protobuf::Protobuf;
use prost::Message;

/// SignatureAndData contains a signature and the data signed over to create that
/// signature.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq)]
pub struct SignatureAndData {
    pub signature: Vec<u8>,
    pub path: Vec<u8>,
    pub data: Vec<u8>,
    pub timestamp: Timestamp,
}
impl core::fmt::Display for SignatureAndData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "signature: {:?}, path: {:?}, data: {:?}, timestamp: {}",
            self.signature, self.path, self.data, self.timestamp
        )
    }
}

impl Protobuf<RawSignatureAndData> for SignatureAndData {}

impl TryFrom<RawSignatureAndData> for SignatureAndData {
    type Error = Error;

    fn try_from(raw: RawSignatureAndData) -> Result<Self, Self::Error> {
        let signature = raw.signature;
        let data = raw.data;
        let timestamp =
            Timestamp::from_nanoseconds(raw.timestamp).map_err(Error::ParseTimeError)?;
        let path = raw.path;
        Ok(Self {
            signature,
            path,
            data,
            timestamp,
        })
    }
}

impl From<SignatureAndData> for RawSignatureAndData {
    fn from(value: SignatureAndData) -> Self {
        Self {
            signature: value.signature,
            path: value.path.encode_to_vec(),
            data: value.data,
            timestamp: value.timestamp.nanoseconds(),
        }
    }
}
