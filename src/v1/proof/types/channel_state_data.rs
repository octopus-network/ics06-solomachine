use crate::prelude::*;
use crate::v1::error::Error;
use ibc_proto::ibc::core::channel::v1::Channel as RawChannel;
use ibc_proto::ibc::lightclients::solomachine::v1::ChannelStateData as RawChannelStateData;
use ibc_proto::protobuf::Protobuf;
use prost::Message;

pub const SOLOMACHINE_CHANNEL_STATE_DATA_TYPE_URL: &str =
    "/ibc.lightclients.solomachine.v2.ChannelStateData";

/// ChannelStateData returns the SignBytes data for channel state
/// verification.
#[derive(Clone, PartialEq)]
pub struct ChannelStateData {
    pub path: Vec<u8>,
    pub channel: Vec<u8>,
}
impl Protobuf<RawChannelStateData> for ChannelStateData {}

impl TryFrom<RawChannelStateData> for ChannelStateData {
    type Error = Error;

    fn try_from(raw: RawChannelStateData) -> Result<Self, Self::Error> {
        Ok(Self {
            path: raw.path,
            channel: raw.channel.ok_or(Error::ChannelEndIsEmpty)?.encode_to_vec(),
        })
    }
}

impl From<ChannelStateData> for RawChannelStateData {
    fn from(value: ChannelStateData) -> Self {
        Self {
            path: value.path,
            channel: Some(RawChannel::decode(&*value.channel).expect("decode channel failed")),
        }
    }
}
