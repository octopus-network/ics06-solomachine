use crate::error::Error;
use crate::prelude::*;
use ibc::core::timestamp::Timestamp;
use ibc_proto::ibc::lightclients::solomachine::v3::SignatureAndData as RawSignatureAndData;
use ibc_proto::protobuf::Protobuf;

/// SignatureAndData contains a signature and the data signed over to create that
/// signature.
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
            "signature: {:?}, path: {}, data: {:?}, timestamp: {}",
            self.signature,
            String::from_utf8(self.path)?,
            self.data,
            self.timestamp
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
        Ok(Self {
            signature,
            path: raw.path,
            data,
            timestamp,
        })
    }
}

impl From<SignatureAndData> for RawSignatureAndData {
    fn from(value: SignatureAndData) -> Self {
        Self {
            signature: value.signature,
            path: value.path,
            data: value.data,
            timestamp: value.timestamp.nanoseconds(),
        }
    }
}
