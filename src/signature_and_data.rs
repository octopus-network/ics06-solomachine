use core::str::FromStr;

use crate::error::Error;
use crate::prelude::*;
use ibc::core::ics24_host::path::Path;
use ibc::core::timestamp::Timestamp;
use ibc_proto::ibc::lightclients::solomachine::v3::SignatureAndData as RawSignatureAndData;
use ibc_proto::protobuf::Protobuf;

/// SignatureAndData contains a signature and the data signed over to create that
/// signature.
#[derive(Clone, PartialEq)]
pub struct SignatureAndData {
    pub signature: Vec<u8>,
    pub path: Path,
    pub data: Vec<u8>,
    pub timestamp: Timestamp,
}
impl core::fmt::Display for SignatureAndData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "signature: {:?}, path: {}, data: {:?}, timestamp: {}",
            self.signature, self.path, self.data, self.timestamp
        )
    }
}

impl Protobuf<RawSignatureAndData> for SignatureAndData {}

impl TryFrom<RawSignatureAndData> for SignatureAndData {
    type Error = Error;

    fn try_from(raw: RawSignatureAndData) -> Result<Self, Self::Error> {
        let signature = raw.signature;
        let path = String::from_utf8(raw.path)
            .map_err(|e| Error::Other(format!("decode Vec<u8> to String failed error({})", e)))?;
        let path =
            Path::from_str(&path).map_err(|e| Error::Other(format!("Parse path error({})", e)))?;
        let data = raw.data;
        let timestamp =
            Timestamp::from_nanoseconds(raw.timestamp).map_err(Error::ParseTimeError)?;
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
            path: format!("{}", value.path).as_bytes().to_vec(),
            data: value.data,
            timestamp: value.timestamp.nanoseconds(),
        }
    }
}
