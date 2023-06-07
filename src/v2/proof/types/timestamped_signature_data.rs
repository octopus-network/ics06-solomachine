use crate::prelude::*;
use crate::v2::error::Error;
use crate::v2::proof::types::signature_and_data::SignatureAndData;
use ibc::core::timestamp::Timestamp;
use ibc_proto::ibc::lightclients::solomachine::v2::TimestampedSignatureData as RawTimestampedSignatureData;
use ibc_proto::protobuf::Protobuf;

pub const SOLOMACHINE_TIMESTAMPED_SIGNATURE_DATA_TYPE_URL: &str =
    "/ibc.lightclients.solomachine.v2.TimestampedSignatureData";

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
