use crate::prelude::*;
use crate::v3::error::Error;
use ibc::core::timestamp::Timestamp;
use ibc_proto::cosmos::tx::signing::v1beta1::signature_descriptor;
use ibc_proto::ibc::lightclients::solomachine::v3::TimestampedSignatureData as RawTimestampedSignatureData;
use ibc_proto::protobuf::Protobuf;

/// TimestampedSignatureData contains the signature data and the timestamp of the
/// signature.
#[derive(Clone, PartialEq)]
pub struct TimestampedSignatureData {
    pub signature_data: signature_descriptor::Data,
    pub timestamp: Timestamp,
}

impl Protobuf<RawTimestampedSignatureData> for TimestampedSignatureData {}

impl TryFrom<RawTimestampedSignatureData> for TimestampedSignatureData {
    type Error = Error;

    fn try_from(raw: RawTimestampedSignatureData) -> Result<Self, Self::Error> {
        Ok(Self {
            signature_data: prost::Message::decode(raw.signature_data.as_slice())
                .map_err(|e| Error::Other(format!("decode SignatureAndData Error({})", e)))?,
            timestamp: Timestamp::from_nanoseconds(raw.timestamp).map_err(Error::ParseTimeError)?,
        })
    }
}

impl From<TimestampedSignatureData> for RawTimestampedSignatureData {
    fn from(value: TimestampedSignatureData) -> Self {
        let mut sig = Vec::new();
        prost::Message::encode(&value.signature_data, &mut sig)
            .expect("encode TimestampedSignatureData");
        Self {
            signature_data: sig,
            timestamp: value.timestamp.nanoseconds(),
        }
    }
}
