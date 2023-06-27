use crate::prelude::*;
use crate::v1::error::Error;
use ibc_proto::ibc::lightclients::solomachine::v1::NextSequenceRecvData as RawNextSequenceRecvData;
use ibc_proto::protobuf::Protobuf;
use prost::Message;

pub const SOLOMACHINE_NEXT_SEQUENCE_RECV_DATA_TYPE_URL: &str =
    "/ibc.lightclients.solomachine.v2.NextSequenceRecvData";

/// NextSequenceRecvData returns the SignBytes data for verification of the next
/// sequence to be received.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct NextSequenceRecvData {
    pub path: Vec<u8>,
    pub next_seq_recv: Vec<u8>,
}

impl Protobuf<RawNextSequenceRecvData> for NextSequenceRecvData {}

impl TryFrom<RawNextSequenceRecvData> for NextSequenceRecvData {
    type Error = Error;

    fn try_from(raw: RawNextSequenceRecvData) -> Result<Self, Self::Error> {
        Ok(Self {
            path: raw.path,
            next_seq_recv: raw.next_seq_recv.encode_to_vec(),
        })
    }
}

impl From<NextSequenceRecvData> for RawNextSequenceRecvData {
    fn from(value: NextSequenceRecvData) -> Self {
        Self {
            path: value.path,
            next_seq_recv: u64::decode(&*value.next_seq_recv).expect("decode next seq recv failed"),
        }
    }
}
