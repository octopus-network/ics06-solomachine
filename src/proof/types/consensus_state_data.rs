use crate::consensus_state::ConsensusState;
use crate::error::Error;
use crate::prelude::*;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::solomachine::v2::ConsensusStateData as RawConsensusStateData;
use ibc_proto::protobuf::Protobuf;

pub const SOLOMACHINE_CONSENSUS_STATE_DATA_TYPE_URL: &str =
    "/ibc.lightclients.solomachine.v1.ConsensusStateData";

/// ConsensusStateData returns the SignBytes data for consensus state
/// verification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct ConsensusStateData {
    pub path: Vec<u8>,
    // ics06 solomachine client consensus state
    pub consensus_state: Vec<u8>,
}

impl Protobuf<RawConsensusStateData> for ConsensusStateData {}

impl TryFrom<RawConsensusStateData> for ConsensusStateData {
    type Error = Error;

    fn try_from(raw: RawConsensusStateData) -> Result<Self, Self::Error> {
        Ok(Self {
            path: raw.path,
            consensus_state: raw
                .consensus_state
                .ok_or(Error::ConsensusStateIsEmpty)?
                .value,
        })
    }
}

impl From<ConsensusStateData> for RawConsensusStateData {
    fn from(value: ConsensusStateData) -> Self {
        Self {
            path: value.path,
            consensus_state: Some(Any {
                type_url: SOLOMACHINE_CONSENSUS_STATE_DATA_TYPE_URL.to_string(),
                value: value.consensus_state,
            }),
        }
    }
}
