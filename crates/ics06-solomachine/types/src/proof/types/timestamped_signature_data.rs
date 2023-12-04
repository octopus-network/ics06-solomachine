use crate::error::Error;
use crate::proof::types::signature_and_data::SignatureAndData;
use ibc_core::primitives::Timestamp;
use ibc_proto::ibc::lightclients::solomachine::v3::TimestampedSignatureData as RawTimestampedSignatureData;
use ibc_proto::Protobuf;

/// TimestampedSignatureData contains the signature data and the timestamp of the
/// signature.
#[derive(Clone, PartialEq)]
pub struct TimestampedSignatureData {
    pub signature_data: SignatureAndData,
    pub timestamp: Timestamp,
}

impl Protobuf<RawTimestampedSignatureData> for TimestampedSignatureData {}

impl TryFrom<RawTimestampedSignatureData> for TimestampedSignatureData {
    type Error = Error;

    fn try_from(raw: RawTimestampedSignatureData) -> Result<Self, Self::Error> {
        Ok(Self {
            signature_data: SignatureAndData::decode_vec(&raw.signature_data)
                .map_err(|e| Error::Other(format!("decode SignatureAndData Error({})", e)))?,
            timestamp: Timestamp::from_nanoseconds(raw.timestamp).map_err(Error::ParseTimeError)?,
        })
    }
}

impl From<TimestampedSignatureData> for RawTimestampedSignatureData {
    fn from(value: TimestampedSignatureData) -> Self {
        Self {
            signature_data: value.signature_data.encode_vec(),
            timestamp: value.timestamp.nanoseconds(),
        }
    }
}
