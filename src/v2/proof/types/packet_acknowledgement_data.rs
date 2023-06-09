use crate::prelude::*;
use crate::v2::error::Error;
use ibc_proto::ibc::lightclients::solomachine::v2::PacketAcknowledgementData as RawPacketAcknowledgementData;
use ibc_proto::protobuf::Protobuf;

pub const SOLOMACHINE_PACKET_ACKNOWLEDGEMENT_DATA_TYPE_URL: &str =
    "/ibc.lightclients.solomachine.v2.PacketAcknowledgementData";

/// PacketAcknowledgementData returns the SignBytes data for acknowledgement
/// verification.
#[derive(Clone, PartialEq)]
pub struct PacketAcknowledgementData {
    pub path: Vec<u8>,
    pub acknowledgement: Vec<u8>,
}

impl Protobuf<RawPacketAcknowledgementData> for PacketAcknowledgementData {}

impl TryFrom<RawPacketAcknowledgementData> for PacketAcknowledgementData {
    type Error = Error;

    fn try_from(raw: RawPacketAcknowledgementData) -> Result<Self, Self::Error> {
        Ok(Self {
            path: raw.path,
            acknowledgement: raw.acknowledgement,
        })
    }
}

impl From<PacketAcknowledgementData> for RawPacketAcknowledgementData {
    fn from(value: PacketAcknowledgementData) -> Self {
        Self {
            path: value.path,
            acknowledgement: value.acknowledgement,
        }
    }
}
