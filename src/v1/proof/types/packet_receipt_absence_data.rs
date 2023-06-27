use crate::prelude::*;
use crate::v1::error::Error;
use ibc_proto::ibc::lightclients::solomachine::v1::PacketReceiptAbsenceData as RawPacketReceiptAbsenceData;
use ibc_proto::protobuf::Protobuf;

pub const SOLOMACHINE_PACKET_RECEIPT_ABSENCE_DATA_TYPE_URL: &str =
    "/ibc.lightclients.solomachine.v2.PacketReceiptAbsenceData";

/// PacketReceiptAbsenceData returns the SignBytes data for
/// packet receipt absence verification.
#[derive(Clone, PartialEq)]
pub struct PacketReceiptAbsenceData {
    pub path: Vec<u8>,
}

impl Protobuf<RawPacketReceiptAbsenceData> for PacketReceiptAbsenceData {}

impl TryFrom<RawPacketReceiptAbsenceData> for PacketReceiptAbsenceData {
    type Error = Error;

    fn try_from(raw: RawPacketReceiptAbsenceData) -> Result<Self, Self::Error> {
        Ok(Self { path: raw.path })
    }
}

impl From<PacketReceiptAbsenceData> for RawPacketReceiptAbsenceData {
    fn from(value: PacketReceiptAbsenceData) -> Self {
        Self { path: value.path }
    }
}
