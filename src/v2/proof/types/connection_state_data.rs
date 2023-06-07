use crate::prelude::*;
use crate::v2::error::Error;
use ibc_proto::ibc::core::connection::v1::ConnectionEnd as RawConnectionEnd;
use ibc_proto::ibc::lightclients::solomachine::v2::ConnectionStateData as RawConnectionStateData;
use ibc_proto::protobuf::Protobuf;
use prost::Message;

/// ConnectionStateData returns the SignBytes data for connection state
/// verification.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq)]
pub struct ConnectionStateData {
    pub path: Vec<u8>,
    pub connection: Vec<u8>,
}

impl Protobuf<RawConnectionStateData> for ConnectionStateData {}

impl TryFrom<RawConnectionStateData> for ConnectionStateData {
    type Error = Error;

    fn try_from(raw: RawConnectionStateData) -> Result<Self, Self::Error> {
        Ok(Self {
            path: raw.path,
            connection: raw
                .connection
                .ok_or(Error::ConnectionEndIsEmpty)?
                .encode_to_vec(),
        })
    }
}

impl From<ConnectionStateData> for RawConnectionStateData {
    fn from(value: ConnectionStateData) -> Self {
        Self {
            path: value.path,
            connection: Some(
                RawConnectionEnd::decode(&*value.connection).expect("Decode connectionEnd Failed"),
            ),
        }
    }
}
